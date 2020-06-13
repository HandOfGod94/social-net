use diesel::prelude::*;
use diesel::{Insertable, QueryResult, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::PooledPgConnection;

use super::handler::RequestBody;
use super::repository::UserCreator;
use super::repository::UserReader;

#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl UserReader for User {
    fn read_all(conn: &PooledPgConnection) -> Vec<User> {
        users.load::<User>(conn).unwrap()
    }
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

impl UserCreator for NewUser {
    fn create(&self, conn: &PooledPgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table).values(self).get_result(conn)
    }
}
