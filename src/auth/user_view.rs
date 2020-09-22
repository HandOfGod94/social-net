use std::collections::HashMap;

use serde_json::{json, Value};

use super::user::User;

pub fn user_list(users: &[User]) -> Value {
  let users_json: Vec<HashMap<String, String>> = users
    .iter()
    .map(|user| {
      let mut resp = HashMap::new();
      resp.insert("id".to_string(), user.id.to_string());
      resp.insert("username".to_string(), user.username.clone());
      resp.insert("email".to_string(), user.email.clone());
      resp
    })
    .collect();

  serde_json::to_value(users_json).unwrap()
}

pub fn user_details(user: &User) -> Value {
  json!({
      "id": user.id,
      "username": user.username,
      "password": "*****",
      "email": user.email
  })
}

pub fn user_create(user: &User) -> Value {
  json!({ "id": user.id })
}
