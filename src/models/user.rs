use diesel::prelude::*;
use diesel::{Insertable, QueryResult, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::handlers::user;
use crate::handlers::user::RequestBody;
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::PooledPgConnection;

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

impl User {
    pub fn fetch_all(conn: &PooledPgConnection) -> Vec<User> {
        users.load::<User>(conn).unwrap()
    }
}

impl NewUser {
    pub fn save(self, conn: &PooledPgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table).values(self).get_result(conn)
    }
}

impl From<user::RequestBody> for NewUser {
    fn from(req: RequestBody) -> Self {
        NewUser {
            username: req.username,
            password: req.password,
            email: req.email,
        }
    }
}
