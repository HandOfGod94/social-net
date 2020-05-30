#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod echo;
mod ping;
mod router;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("Starting Server");

    warp::serve(router::routes())
        .run(([127, 0, 0, 1], 8080))
        .await;
}
