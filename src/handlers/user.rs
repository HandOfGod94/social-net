use std::convert::Infallible;

use diesel::prelude::*;
use warp::reply::Json;
use warp::Filter;
use warp::{get, path};

use crate::models::user::User;
use crate::schema::users::dsl::*;
use crate::{ConnectionPool, PooledPgConnection};

pub fn routes(
    pool: ConnectionPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let user_index_route = path!("users")
        .and(get())
        .and(with_db(pool))
        .and_then(user_index);

    user_index_route
}

fn with_db(
    pool: ConnectionPool,
) -> impl Filter<Extract = (PooledPgConnection,), Error = Infallible> + Clone {
    warp::any().map(move || pool.get().unwrap())
}

async fn user_index(conn: PooledPgConnection) -> Result<Json, Infallible> {
    let resp = users.load::<User>(&conn).unwrap();
    Ok(warp::reply::json(&resp))
}

#[cfg(test)]
mod test {
    use std::env;

    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::PgConnection;
    use serde_json::json;
    use warp::http::StatusCode;

    use super::*;

    fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        Pool::new(manager).expect("Postgres connection pool couldn't be created")
    }

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
        assert_eq!(*resp.body(), expected_response, "should return list of users");
    }
}
