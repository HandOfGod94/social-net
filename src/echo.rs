use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use warp::reply::Json;
use warp::{Filter, Rejection};

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    data: HashMap<String, String>,
}

pub fn json_body() -> impl Filter<Extract = (RequestBody,), Error = Rejection> + Clone {
    warp::body::json()
}

pub async fn handler(req: RequestBody) -> Result<Json, Infallible> {
    Ok(warp::reply::json(&req))
}
