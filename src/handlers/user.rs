use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use warp::http::StatusCode;
use warp::reply::{json, with_status, Json, WithStatus};
use warp::{get, path};
use warp::{post, Filter, Rejection};

use crate::models::user::{NewUser, User};
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
    username: String,
    password: String,
    email: String,
}

async fn user_index(conn: PooledPgConnection) -> Result<Json, Infallible> {
    let resp = User::fetch_all(&conn);
    Ok(json(&resp))
}

async fn user_create(conn: PooledPgConnection, req: RequestBody) -> Result<WithStatus<Json>, Infallible> {
    let new_user = NewUser{
        username: req.username,
        password: req.password,
        email: req.email,
    };

    match new_user.save(&conn) {
        Ok(user) => Ok(with_status(json(&user), StatusCode::CREATED)),
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
    use super::*;
    use crate::test_helpers::establish_connection;
    use serde_json::json;
    use warp::http::StatusCode;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_user_index() {
        let db = establish_connection();
        let filter = routes(db.clone());
        let resp = warp::test::request().method("GET").path("/users").reply(&filter).await;

        assert_eq!(resp.status(), StatusCode::OK);
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
            "username": "bob",
            "email": "bob@open.org",
            "password": "password"
        });
        let expected_resp_body: HashMap<String, String> = serde_json::from_value(expected_response).unwrap();
        let actual_resp_body: HashMap<String, String> = serde_json::from_str(std::str::from_utf8(&*resp.body()).unwrap()).unwrap();

        assert_eq!(resp.status(), StatusCode::CREATED);
        assert!(actual_resp_body.contains_key("id"));
        assert_eq!(expected_resp_body.get("username"), actual_resp_body.get("username"));
        assert_eq!(expected_resp_body.get("password"), actual_resp_body.get("password"));
        assert_eq!(expected_resp_body.get("email"), actual_resp_body.get("email"));
    }
}
