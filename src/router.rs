use std::env;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use warp::{Filter, Reply};

use crate::handlers;
use crate::user;
use crate::ConnectionPool;

fn establish_connection() -> ConnectionPool {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::new(manager).expect("Postgres connection pool couldn't be created")
}

pub fn routes() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    let db_pool = establish_connection();
    handlers::ping::routes()
        .or(handlers::echo::routes())
        .or(user::handler::routes(db_pool))
}
