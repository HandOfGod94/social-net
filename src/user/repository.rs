use diesel::prelude::*;
use diesel::QueryResult;
#[cfg(test)]
use mockall::automock;

use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::user::model::NewUser;
use crate::ConnectionPool;

use super::model::User;
use uuid::Uuid;

#[cfg_attr(test, automock)]
pub trait Repository {
    fn read_all(&self) -> Vec<User>;
    fn create(&self, new_user: NewUser) -> QueryResult<User>;
    fn find(&self, user_id: Uuid) -> QueryResult<User>;
}

#[derive(Clone)]
pub struct UserRepo {
    pool: ConnectionPool,
}

impl UserRepo {
    pub fn new(pool: ConnectionPool) -> UserRepo {
        UserRepo { pool }
    }
}

impl Repository for UserRepo {
    fn read_all(&self) -> Vec<User> {
        let conn = self.pool.get().unwrap();
        users.load::<User>(&conn).unwrap()
    }

    fn create(&self, new_user: NewUser) -> QueryResult<User> {
        let conn = self.pool.get().unwrap();
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(&conn)
    }

    fn find(&self, user_id: Uuid) -> QueryResult<User> {
        let conn = self.pool.get().unwrap();
        users.find(user_id).first(&conn)
    }
}
