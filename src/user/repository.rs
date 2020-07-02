use diesel::prelude::*;
use diesel::QueryResult;
use uuid::Uuid;

use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::user::model::NewUser;

use super::model::User;

pub struct UserRepo;

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

    pub fn delete(conn: &PgConnection, user_id: Uuid) -> QueryResult<usize> {
        diesel::delete(users.filter(id.eq(user_id))).execute(conn)
    }
}

#[cfg(test)]
mod tests {
    use diesel::{PgConnection, RunQueryDsl};
    use fake::faker::internet::en::FreeEmail;
    use fake::faker::internet::en::Password;
    use fake::faker::name::en::Name;
    use fake::Fake;
    use uuid::Uuid;

    use crate::schema::users;
    use crate::test_helpers::establish_connection;
    use crate::user::model::{NewUser, User};
    use crate::user::repository::UserRepo;

    fn create_fake_users(conn: &PgConnection) -> User {
        let user = NewUser {
            username: Name().fake(),
            password: Password(5..10).fake(),
            email: FreeEmail().fake(),
        };
        let user = diesel::insert_into(users::table)
            .values(&user)
            .get_result(conn)
            .expect("Failed to create fake user");
        user
    }

    #[test]
    fn read_all_returns_all_users() {
        let conn = establish_connection().get().unwrap();
        let bob = create_fake_users(&conn);
        let alice = create_fake_users(&conn);

        let actual = UserRepo::read_all(&conn);
        assert!(actual.contains(&bob));
        assert!(actual.contains(&alice));
        assert_eq!(actual.len(), 2);
    }

    #[test]
    fn creates_users_for_valid_data() {
        let conn = establish_connection().get().unwrap();
        let bob = NewUser {
            username: "bob".to_string(),
            password: "password".to_string(),
            email: "bob@open.org".to_string(),
        };

        let result = UserRepo::create(&conn, bob);
        assert!(result.is_ok());
    }

    #[test]
    fn create_user_returns_error_fo_invalid_data() {
        let conn = establish_connection().get().unwrap();
        let bob = create_fake_users(&conn);
        let bob = NewUser {
            username: bob.username,
            password: bob.password,
            email: bob.email,
        };

        let result = UserRepo::create(&conn, bob);
        assert!(result.is_err());
    }

    #[test]
    fn find_users_if_is_present() {
        let conn = establish_connection().get().unwrap();
        let bob = create_fake_users(&conn);

        let actual = UserRepo::find(&conn, bob.id).unwrap();
        assert_eq!(actual, bob);
    }

    #[test]
    fn find_returns_errof_if_user_is_absent() {
        let conn = establish_connection().get().unwrap();
        let id = Uuid::new_v4();

        let result = UserRepo::find(&conn, id);
        assert!(result.is_err())
    }

    #[test]
    fn deletes_user_for_valid_id() {
       let conn = establish_connection().get().unwrap();
        let bob = create_fake_users(&conn);

        let result = UserRepo::delete(&conn, bob.id);
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn delete_returns_error_for_non_existent_user() {
       let conn = establish_connection().get().unwrap();
        let id = Uuid::new_v4();

        let result = UserRepo::delete(&conn, id);
        assert_eq!(result, Ok(0));
    }
}
