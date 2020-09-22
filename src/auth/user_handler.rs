use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::user::NewUser;
use crate::ConnectionPool;

use super::user_repo::UserRepo;
use super::user_view;

#[derive(Serialize, Deserialize)]
pub struct RequestBody {
  pub username: String,
  pub password: String,
  pub email: String,
}

#[get("/users")]
pub async fn index(pool: web::Data<ConnectionPool>) -> HttpResponse {
  let conn = pool.get().unwrap();
  let users = web::block(move || UserRepo::read_all(&conn)).await.unwrap();
  let resp = user_view::user_list(&users);
  HttpResponse::Ok().json(resp)
}

#[get("/users/{id}")]
pub async fn show(pool: web::Data<ConnectionPool>, params: web::Path<(Uuid,)>) -> HttpResponse {
  let id = params.into_inner().0;
  let conn = pool.get().unwrap();
  let user_result = web::block(move || UserRepo::find(&conn, id)).await;

  match user_result {
    Ok(user) => {
      let resp = user_view::user_details(&user);
      HttpResponse::Ok().json(resp)
    }
    Err(err) => HttpResponse::NotFound().body(err.to_string()),
  }
}

#[post("/users")]
pub async fn create(pool: web::Data<ConnectionPool>, req: web::Json<RequestBody>) -> HttpResponse {
  let conn = pool.get().unwrap();
  let new_user = req.into_inner().into();
  let user_create_result = web::block(move || UserRepo::create(&conn, new_user)).await;

  match user_create_result {
    Ok(user) => {
      let resp = user_view::user_create(&user);
      HttpResponse::Created().json(resp)
    }
    Err(err) => HttpResponse::UnprocessableEntity().body(err.to_string()),
  }
}
