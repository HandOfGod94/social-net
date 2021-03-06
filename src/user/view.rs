use std::collections::HashMap;

use serde_json::{json, Value};

use super::model::User;

pub fn user_list(users: &[User]) -> Value {
    let users_json: Vec<HashMap<String, String>> = users
        .iter()
        .map(|user| {
            let mut resp = HashMap::new();
            resp.insert("id".to_string(), user.id.to_string());
            resp.insert("username".to_string(), user.username.clone());
            resp.insert("email".to_string(), user.email.clone());
            resp
        })
        .collect();

    serde_json::to_value(users_json).unwrap()
}

pub fn user_details(user: &User) -> Value {
    json!({
        "id": user.id,
        "username": user.username,
        "password": "*****",
        "email": user.email
    })
}

pub fn user_create(user: &User) -> Value {
    json!({ "id": user.id })
}

#[cfg(test)]
mod tests {
    use fake::faker::internet::en::FreeEmail;
    use fake::faker::internet::en::Password;
    use fake::faker::name::en::Name;
    use fake::Fake;
    use serde_json::json;
    use uuid::Uuid;

    use super::*;

    fn create_fake_users() -> User {
        User {
            id: Uuid::new_v4(),
            username: Name().fake(),
            password: Password(5..10).fake(),
            email: FreeEmail().fake(),
        }
    }

    #[test]
    fn user_list_view_returns_id_username_and_email() {
        let bob = create_fake_users();
        let alice = create_fake_users();
        let users = vec![bob.clone(), alice.clone()];

        let actual = user_list(&users);
        let expected = json!([
            {
                "id": users[0].id,
                "username": users[0].username,
                "email": users[0].email
            }, {
                "id": users[1].id,
                "username": users[1].username,
                "email": users[1].email
            }
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn user_create_view_returns_id() {
        let bob = create_fake_users();
        let actual = user_create(&bob);

        let expected = json!({ "id": bob.id });

        assert_eq!(actual, expected)
    }

    #[test]
    fn user_details_view_returns_user_details() {
        let bob = create_fake_users();
        let expected = json!({
            "id": bob.id,
            "username": bob.username,
            "password": "*****",
            "email": bob.email
        });

        let actual = user_details(&bob);
        assert_eq!(expected, actual)
    }
}
