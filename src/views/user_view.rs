use serde_json::{json, Value};

use crate::models::user::User;

pub fn user_create(user: User) -> Value {
    let id = user.id;
    json!({ "id": id })
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_user_create_view_returns_id() {
        let new_id = Uuid::new_v4();
        let user = User {
            id: new_id,
            username: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
        };
        let resp = user_create(user);

        assert_eq!(resp["id"], new_id.to_string())
    }
}
