use diesel::prelude::*;
use diesel::QueryResult;
use uuid::Uuid;

use crate::auth::user::NewUser;
use crate::schema::users;
use crate::schema::users::dsl::*;

use super::user::User;

pub struct UserRepo;

impl UserRepo {
  pub fn read_all(conn: &PgConnection) -> QueryResult<Vec<User>> {
    users.load::<User>(conn)
  }

  pub fn create(conn: &PgConnection, new_user: NewUser) -> QueryResult<User> {
    diesel::insert_into(users::table)
      .values(new_user)
      .get_result(conn)
  }

  pub fn find(conn: &PgConnection, user_id: Uuid) -> QueryResult<User> {
    users.find(user_id).first(conn)
  }

  pub fn delete(conn: &PgConnection, user_id: Uuid) -> QueryResult<usize> {
    diesel::delete(users.filter(id.eq(user_id))).execute(conn)
  }
}
