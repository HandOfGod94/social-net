use std::collections::HashMap;
use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use warp::reply::Json;
use warp::{path, post};
use warp::{Filter, Rejection, Reply};

pub fn routes(
) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    path!("echo").and(post()).and(json_body()).and_then(handler)
}

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    data: HashMap<String, String>,
}

pub fn json_body(
) -> impl Filter<Extract = (RequestBody,), Error = Rejection> + Clone {
    warp::body::json()
}

pub async fn handler(req: RequestBody) -> Result<Json, Infallible> {
    Ok(warp::reply::json(&req))
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use warp::http::StatusCode;

    use super::*;

    #[tokio::test]
    async fn test_handler() {
        let mut data = HashMap::new();
        data.insert("foo".to_string(), "bar".to_string());
        let req = RequestBody { data };

        let filter = routes();
        let resp = warp::test::request()
            .method("POST")
            .path("/echo")
            .json(&req)
            .reply(&filter)
            .await;

        let expected_resp = json!({
            "data": {
                "foo": "bar"
            }
        })
        .to_string();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(*resp.body(), expected_resp)
    }
}
