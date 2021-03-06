#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use warp::Filter;

mod echo;
mod ping;
mod router;
mod schema;
mod user;

type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let log = warp::log("social_net");
    let router = router::routes().with(log);

    info!("Starting Server");

    warp::serve(router).run(([127, 0, 0, 1], 8080)).await;
}

#[cfg(test)]
mod test_helpers;
