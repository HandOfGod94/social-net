use std::env;
use warp::{Filter, Reply};

use crate::handlers;
use diesel::{Connection, PgConnection};

fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn routes() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    handlers::ping::routes().or(handlers::echo::routes())
}
