#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use actix_web::{App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

mod auth;
mod echo;
mod ping;
mod schema;
// mod user;
mod router;

type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().configure(router::route_config))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod test_helpers;
