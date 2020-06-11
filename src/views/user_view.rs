use serde_json::{json, Value};

use crate::models::user::User;

pub fn user_create(user: User) -> Value {
    let id = user.id;
    json!({ "id": id })
}

#[cfg(test)]
mod tests {
    use fake::faker::internet::en::FreeEmail;
    use fake::faker::internet::en::Password;
    use fake::faker::name::en::Name;
    use uuid::Uuid;

    use super::*;
    use fake::Fake;

    #[test]
    fn user_create_view_returns_id() {
        let new_id = Uuid::new_v4();
        let user = User {
            id: new_id,
            username: Name().fake(),
            email: FreeEmail().fake(),
            password: Password(5..10).fake(),
        };
        let resp = user_create(user);

        assert_eq!(resp["id"], new_id.to_string())
    }
}
