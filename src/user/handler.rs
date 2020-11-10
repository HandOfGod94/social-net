use std::convert::Infallible;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::{json, with_status, Json, WithStatus};
use warp::{delete, get, path, post, Filter, Rejection};

use crate::user::repository::UserRepo;
use crate::ConnectionPool;

use super::model::NewUser;
use super::view;

pub fn routes(
    pool: ConnectionPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let user_index_route = path!("users")
        .and(get())
        .and(with_db_conn(pool.clone()))
        .and_then(user_index);

    let user_details_route = path!("users" / Uuid)
        .and(get())
        .and(with_db_conn(pool.clone()))
        .and_then(user_details);

    let user_create_route = path!("users")
        .and(post())
        .and(with_db_conn(pool.clone()))
        .and(json_body())
        .and_then(user_create);

    let user_delete_route = path!("users" / Uuid)
        .and(delete())
        .and(with_db_conn(pool))
        .and_then(user_delete);

    user_index_route
        .or(user_details_route)
        .or(user_create_route)
        .or(user_delete_route)
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    pub username: String,
    pub password: String,
    pub email: String,
}

async fn user_index(
    conn: PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Json, Infallible> {
    let users = UserRepo::read_all(&conn);
    let resp = view::user_list(&users);
    Ok(json(&resp))
}

async fn user_create(
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    req: RequestBody,
) -> Result<WithStatus<Json>, Infallible> {
    let new_user: NewUser = req.into();
    let result = UserRepo::create(&conn, new_user);

    match result {
        Ok(user) => {
            let resp = view::user_create(&user);
            Ok(with_status(json(&resp), StatusCode::CREATED))
        }
        Err(err) => Ok(with_status(
            json(&err.to_string()),
            StatusCode::UNPROCESSABLE_ENTITY,
        )),
    }
}

async fn user_details(
    id: Uuid,
    conn: PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<WithStatus<Json>, Infallible> {
    match UserRepo::find(&conn, id) {
        Ok(user) => {
            let resp = view::user_details(&user);
            Ok(with_status(json(&resp), StatusCode::OK))
        }
        Err(err) => {
            Ok(with_status(json(&err.to_string()), StatusCode::NOT_FOUND))
        }
    }
}

async fn user_delete(
    id: Uuid,
    conn: PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<WithStatus<Json>, Infallible> {
    match UserRepo::delete(&conn, id) {
        Ok(val) if val > 0 => Ok(with_status(
            json(&"{\"success\": true}".to_string()),
            StatusCode::OK,
        )),
        Ok(_) => Ok(with_status(
            json(&"{\"success\": false}".to_string()),
            StatusCode::NOT_FOUND,
        )),
        Err(err) => {
            error!("Something went really wrong while deleting user");
            Ok(with_status(
                json(&err.to_string()),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

fn with_db_conn(
    pool: ConnectionPool,
) -> impl Filter<
    Extract = (PooledConnection<ConnectionManager<PgConnection>>,),
    Error = Infallible,
> + Clone {
    warp::any().map(move || pool.get().unwrap())
}

fn json_body(
) -> impl Filter<Extract = (RequestBody,), Error = Rejection> + Clone {
    warp::body::json()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use diesel::RunQueryDsl;
    use fake::faker::internet::en::{FreeEmail, Password};
    use fake::faker::name::en::Name;
    use fake::Fake;
    use serde_json::json;
    use warp::http::StatusCode;
    use warp::test::request;
    use warp::Reply;

    use crate::schema::users;
    use crate::test_helpers::establish_connection;
    use crate::user::model::User;

    use super::*;

    fn create_fake_users(conn: &PgConnection) -> User {
        let user = NewUser {
            username: Name().fake(),
            password: Password(5..10).fake(),
            email: FreeEmail().fake(),
        };
        let user = diesel::insert_into(users::table)
            .values(&user)
            .get_result(conn)
            .expect("Failed to create fake user");
        user
    }

    #[tokio::test]
    async fn test_user_index() {
        let db = establish_connection();
        let filter = routes(db.clone());
        let resp = request().method("GET").path("/users").reply(&filter).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn post_user_succeeds_for_valid_values() {
        let db = establish_connection();
        let req = RequestBody {
            username: Name().fake(),
            email: FreeEmail().fake(),
            password: Password(5..10).fake(),
        };

        let filter = routes(db.clone());
        let resp = request()
            .method("POST")
            .path("/users")
            .json(&req)
            .reply(&filter)
            .await;

        let actual_resp_body: HashMap<String, String> =
            std::str::from_utf8(resp.body())
                .map(|body| serde_json::from_str(body).unwrap())
                .expect("Invalid response");

        assert_eq!(resp.status(), StatusCode::CREATED);
        assert!(actual_resp_body.contains_key("id"));
        assert_eq!(actual_resp_body.keys().len(), 1);
    }

    #[tokio::test]
    async fn user_create_fails_for_duplicate_username() {
        let conn = establish_connection().get().unwrap();
        let user = create_fake_users(&conn);
        let new_user_request = RequestBody {
            username: user.username,
            password: user.password,
            email: user.email,
        };

        let (parts, body) = user_create(conn, new_user_request)
            .await
            .unwrap()
            .into_response()
            .into_parts();
        let body = hyper::body::to_bytes(body).await.unwrap();
        assert_eq!(parts.status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(body, "\"duplicate key value violates unique constraint \\\"users_username_key\\\"\"")
    }

    #[tokio::test]
    async fn user_index_returns_json_array() {
        let pool = establish_connection();
        let conn = pool.get().unwrap();

        let bob = create_fake_users(&conn);
        let alice = create_fake_users(&conn);
        let expected = json!([
            {
                "email": bob.email,
                "id": bob.id,
                "username": bob.username
            },{
                "email": alice.email,
                "id": alice.id,
                "username": alice.username
            }
        ])
        .to_string();

        let result = user_index(conn).await.unwrap().into_response();
        let actual = hyper::body::to_bytes(result.into_body()).await.unwrap();

        assert_eq!(actual, expected)
    }

    #[tokio::test]
    async fn user_details_returns_user_json_value() {
        let pool = establish_connection();
        let conn = pool.get().unwrap();
        let bob = create_fake_users(&conn);
        let bob_id = bob.clone().id;
        let expected = json!({
            "id": bob_id,
            "username": bob.clone().username,
            "password": "*****",
            "email": bob.clone().email
        })
        .to_string();

        let user_details =
            user_details(bob_id, conn).await.unwrap().into_response();
        let actual = hyper::body::to_bytes(user_details.into_body())
            .await
            .unwrap();

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn user_details_returns_error_for_non_existent_user() {
        let conn = establish_connection().get().unwrap();
        let uuid = Uuid::new_v4();
        let (parts, body) = user_details(uuid, conn)
            .await
            .unwrap()
            .into_response()
            .into_parts();
        let body = hyper::body::to_bytes(body).await.unwrap();

        assert_eq!(parts.status, StatusCode::NOT_FOUND);
        assert_eq!(body, "\"NotFound\"");
    }

    #[tokio::test]
    async fn delete_returns_success_message_if_user_exist() {
        let conn = establish_connection().get().unwrap();
        let bob = create_fake_users(&conn);
        let (parts, body) = user_delete(bob.id, conn)
            .await
            .unwrap()
            .into_response()
            .into_parts();
        let body = hyper::body::to_bytes(body).await.unwrap();

        assert_eq!(parts.status, StatusCode::OK);
        assert_eq!(body, "\"{\\\"success\\\": true}\"");
    }

    #[tokio::test]
    async fn delete_returns_failure_if_user_does_not_exist() {
        let conn = establish_connection().get().unwrap();
        let id = Uuid::new_v4();
        let (parts, body) = user_delete(id, conn)
            .await
            .unwrap()
            .into_response()
            .into_parts();
        let body = hyper::body::to_bytes(body).await.unwrap();

        assert_eq!(parts.status, StatusCode::NOT_FOUND);
        assert_eq!(body, "\"{\\\"success\\\": false}\"");
    }
}
