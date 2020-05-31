#[macro_use]
extern crate log;
extern crate diesel;
extern crate dotenv;
extern crate pretty_env_logger;

mod handlers;
mod router;
mod schema;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("Starting Server");

    warp::serve(router::routes())
        .run(([127, 0, 0, 1], 8080))
        .await;
}
