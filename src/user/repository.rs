use diesel::prelude::*;
use diesel::QueryResult;
use uuid::Uuid;

use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::user::model::NewUser;

use super::model::User;

#[derive(Clone)]
pub struct UserRepo {}

impl UserRepo {
    pub fn read_all(conn: &PgConnection) -> Vec<User> {
        users.load::<User>(conn).unwrap()
    }

    pub fn create(conn: &PgConnection, new_user: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(conn)
    }

    pub fn find(conn: &PgConnection, user_id: Uuid) -> QueryResult<User> {
        users.find(user_id).first(conn)
    }
}
