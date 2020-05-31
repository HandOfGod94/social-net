#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

mod handlers;
mod models;
mod router;
mod schema;

type ConnectionPool = Pool<ConnectionManager<PgConnection>>;
type PooledPgConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("Starting Server");

    warp::serve(router::routes())
        .run(([127, 0, 0, 1], 8080))
        .await;
}
