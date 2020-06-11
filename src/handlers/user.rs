use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use warp::http::StatusCode;
use warp::reply::{json, with_status, Json, WithStatus};
use warp::{get, path};
use warp::{post, Filter, Rejection};

use crate::models::user::{NewUser, User};
use crate::views::user_view;
use crate::{ConnectionPool, PooledPgConnection};

pub fn routes(pool: ConnectionPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let user_index_route = path!("users")
        .and(get())
        .and(with_db(pool.clone()))
        .and_then(user_index);

    let user_create_route = path!("users")
        .and(post())
        .and(with_db(pool))
        .and(json_body())
        .and_then(user_create);

    user_index_route.or(user_create_route)
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    pub username: String,
    pub password: String,
    pub email: String,
}

async fn user_index(conn: PooledPgConnection) -> Result<Json, Infallible> {
    let users = User::fetch_all(&conn);
    let resp = user_view::user_list(&users);
    Ok(json(&resp))
}

async fn user_create(conn: PooledPgConnection, req: RequestBody) -> Result<WithStatus<Json>, Infallible> {
    let new_user = NewUser::from(req);

    match new_user.save(&conn) {
        Ok(user) => {
            let resp = user_view::user_create(&user);
            Ok(with_status(json(&resp), StatusCode::CREATED))
        }
        Err(err) => Ok(with_status(json(&err.to_string()), StatusCode::BAD_REQUEST)),
    }
}

fn with_db(pool: ConnectionPool) -> impl Filter<Extract = (PooledPgConnection,), Error = Infallible> + Clone {
    warp::any().map(move || pool.get().unwrap())
}

fn json_body() -> impl Filter<Extract = (RequestBody,), Error = Rejection> + Clone {
    warp::body::json()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use fake::faker::internet::en::FreeEmail;
    use fake::faker::internet::en::Password;
    use fake::faker::name::en::Name;
    use fake::Fake;
    use warp::http::StatusCode;

    use crate::test_helpers::establish_connection;

    use super::*;

    #[tokio::test]
    async fn test_user_index() {
        let db = establish_connection();
        let filter = routes(db.clone());
        let resp = warp::test::request().method("GET").path("/users").reply(&filter).await;

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
        let resp = warp::test::request()
            .method("POST")
            .path("/users")
            .json(&req)
            .reply(&filter)
            .await;

        let actual_resp_body: HashMap<String, String> = std::str::from_utf8(resp.body())
            .map(|body| serde_json::from_str(body).unwrap())
            .expect("Invalid response");

        assert_eq!(resp.status(), StatusCode::CREATED);
        assert!(actual_resp_body.contains_key("id"));
        assert_eq!(actual_resp_body.keys().len(), 1);
    }
}
