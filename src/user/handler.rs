use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use warp::http::StatusCode;
use warp::reply::{json, with_status, Json, WithStatus};
use warp::{get, path};
use warp::{post, Filter, Rejection};

use crate::user::repository::{Repository, UserRepo};
use crate::ConnectionPool;

use super::model::NewUser;
use super::view;

pub fn routes(
    pool: ConnectionPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let user_index_route = path!("users")
        .and(get())
        .and(with_repo(pool.clone()))
        .and_then(user_index);

    let user_create_route = path!("users")
        .and(post())
        .and(with_repo(pool))
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

async fn user_index(user_repo: impl Repository) -> Result<Json, Infallible> {
    let users = user_repo.read_all();
    let resp = view::user_list(&users);
    Ok(json(&resp))
}

async fn user_create(
    user_repo: impl Repository,
    req: RequestBody,
) -> Result<WithStatus<Json>, Infallible> {
    let new_user = NewUser::from(req);
    let result = user_repo.create(new_user);

    match result {
        Ok(user) => {
            let resp = view::user_create(&user);
            Ok(with_status(json(&resp), StatusCode::CREATED))
        }
        Err(err) => {
            Ok(with_status(json(&err.to_string()), StatusCode::BAD_REQUEST))
        }
    }
}

fn with_repo(
    pool: ConnectionPool,
) -> impl Filter<Extract = (impl Repository,), Error = Infallible> + Clone {
    let repo = UserRepo::new(pool);
    warp::any().map(move || repo.clone())
}

fn json_body(
) -> impl Filter<Extract = (RequestBody,), Error = Rejection> + Clone {
    warp::body::json()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use fake::faker::internet::en::FreeEmail;
    use fake::faker::internet::en::Password;
    use fake::faker::name::en::Name;
    use fake::Fake;
    use serde_json::json;
    use uuid::Uuid;
    use warp::http::StatusCode;
    use warp::test::request;
    use warp::Reply;

    use crate::test_helpers::establish_connection;
    use crate::user::model::User;
    use crate::user::repository::*;

    use super::*;

    fn create_fake_users() -> User {
        User {
            id: Uuid::new_v4(),
            username: Name().fake(),
            password: Password(5..10).fake(),
            email: FreeEmail().fake(),
        }
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
    async fn user_index_returns_json_array() {
        let mut mock_user_repo = MockRepository::new();
        let bob = create_fake_users();
        let alice = create_fake_users();
        let expected_body = json!([
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

        mock_user_repo
            .expect_read_all()
            .times(1)
            .returning(move || vec![bob.clone(), alice.clone()]);

        let result = user_index(mock_user_repo).await.unwrap().into_response();
        let body = hyper::body::to_bytes(result.into_body()).await.unwrap();

        assert_eq!(expected_body, body)
    }
}
