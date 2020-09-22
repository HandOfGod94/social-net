use std::env;

use actix_web::web::ServiceConfig;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

use crate::handlers::echo;
use crate::handlers::ping;
// use crate::user;
use crate::ConnectionPool;

fn establish_connection() -> ConnectionPool {
    dotenv::dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::new(manager).expect("Postgres connection pool couldn't be created")
}

pub fn route_config(cfg: &mut ServiceConfig) {
    cfg.service(ping::index).service(echo::index);
}
