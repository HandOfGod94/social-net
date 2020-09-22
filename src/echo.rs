use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
  data: HashMap<String, String>,
}

#[post("/echo")]
pub async fn index(data: web::Json<RequestBody>) -> HttpResponse {
  HttpResponse::Ok().json(&data.data)
}
