use actix_web::{get, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
  success: bool,
}

#[get("/ping")]
pub async fn index() -> HttpResponse {
  HttpResponse::Ok().json(Response { success: true })
}
