use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use warp::http::StatusCode;
use warp::reply::{json, with_status, Json, WithStatus};
use warp::{get, path};
use warp::{post, Filter, Rejection};

use crate::models::user::{NewUser, User};
use crate::{ConnectionPool, PooledPgConnection};

pub fn routes(
    pool: ConnectionPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
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
    username: String,
    password: String,
    email: String,
}

async fn user_index(conn: PooledPgConnection) -> Result<Json, Infallible> {
    let resp = User::fetch_all(&conn);
    Ok(json(&resp))
}

async fn user_create(
    conn: PooledPgConnection,
    req: RequestBody,
) -> Result<WithStatus<Json>, Infallible> {
    let new_user = NewUser {
        username: req.username,
        password: req.password,
        email: req.email,
    };

    match new_user.save(&conn) {
        Ok(user) => Ok(with_status(json(&user), StatusCode::CREATED)),
        Err(err) => Ok(with_status(json(&err.to_string()), StatusCode::BAD_REQUEST)),
    }
}

fn with_db(
    pool: ConnectionPool,
) -> impl Filter<Extract = (PooledPgConnection,), Error = Infallible> + Clone {
    warp::any().map(move || pool.get().unwrap())
}

fn json_body() -> impl Filter<Extract = (RequestBody,), Error = Rejection> + Clone {
    warp::body::json()
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use warp::http::StatusCode;
    use super::*;
    use crate::test_helpers::establish_connection;

    #[tokio::test]
    async fn test_user_index() {
        let db = establish_connection();
        let filter = routes(db.clone());
        let resp = warp::test::request()
            .method("GET")
            .path("/users")
            .reply(&filter)
            .await;

        let expected_response = json!([
            {
                "id": "123",
                "username": "bob",
                "email_id": "bob@open.org"
            }
        ])
        .to_string();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            *resp.body(),
            expected_response,
            "should return list of users"
        );
    }

    #[tokio::test]
    async fn test_user_create() {
        let db = establish_connection();
        let req = RequestBody {
            username: "bob".to_string(),
            email: "bob@open.org".to_string(),
            password: "password".to_string(),
        };

        let filter = routes(db.clone());
        let resp = warp::test::request()
            .method("POST")
            .path("/users")
            .json(&req)
            .reply(&filter)
            .await;

        let expected_response = json!({
            "id": "1234",
            "username": "bob",
            "password": "bob@open.org",
            "email": "password"
        }).to_string();

        assert_eq!(resp.status(), StatusCode::CREATED);
        assert_eq!(
            *resp.body(),
            expected_response,
            "should return list of users"
        );
    }
}
