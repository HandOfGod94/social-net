use diesel::QueryResult;

use crate::PooledPgConnection;

use super::model::User;

// todo: change it to private once all the user functions are moved to same mod
pub trait UserReader {
    fn read_all(conn: &PooledPgConnection) -> Vec<User>;
}

pub trait UserCreator {
    fn create(&self, conn: &PooledPgConnection) -> QueryResult<User>;
}
