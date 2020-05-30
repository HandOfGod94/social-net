use serde::Serialize;
use std::convert::Infallible;
use warp::reply::Json;

#[derive(Serialize)]
struct Response {
    success: bool,
}

pub async fn handler() -> Result<Json, Infallible> {
    let resp = Response { success: true };
    Ok(warp::reply::json(&resp))
}
