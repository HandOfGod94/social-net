use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

use super::handler::RequestBody;

#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
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
