use std::convert::Infallible;

use serde::Serialize;
use warp::reply::Json;
use warp::{get, path, Filter, Reply};

pub fn routes() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    path!("ping").and(get()).and_then(handler)
}

#[derive(Serialize)]
struct Response {
    success: bool,
}

pub async fn handler() -> Result<Json, Infallible> {
    let resp = Response { success: true };
    Ok(warp::reply::json(&resp))
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use warp::http::StatusCode;

    use super::*;

    #[tokio::test]
    async fn test_get_success() {
        let filter = routes();
        let resp = warp::test::request()
            .method("GET")
            .path("/ping")
            .reply(&filter)
            .await;

        let expected_response = json!(
            {
                "success": true
            }
        )
        .to_string();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(*resp.body(), expected_response);
    }

    #[tokio::test]
    async fn test_unsupported_methods() {
        let filter = routes();
        let resp = warp::test::request()
            .method("POST")
            .path("/ping")
            .reply(&filter)
            .await;

        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
}
