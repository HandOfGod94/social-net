use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::user_handler::RequestBody;
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct User {
  pub id: Uuid,
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Insertable, Serialize, PartialEq, Deserialize, Debug)]
#[table_name = "users"]
pub struct NewUser {
  pub username: String,
  pub email: String,
  pub password: String,
}

impl From<RequestBody> for NewUser {
  fn from(req: RequestBody) -> Self {
    NewUser {
      username: req.username,
      password: req.password,
      email: req.email,
    }
  }
}
