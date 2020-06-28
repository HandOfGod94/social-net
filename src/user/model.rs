use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

use super::handler::RequestBody;

#[derive(Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Serialize, PartialEq, Deserialize, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creates_new_user_from_request_body() {
        let req_body = RequestBody {
            username: "Bob".to_string(),
            password: "password".to_string(),
            email: "bob@open.org".to_string(),
        };

        let expected = NewUser {
            username: "Bob".to_string(),
            password: "password".to_string(),
            email: "bob@open.org".to_string(),
        };

        let actual: NewUser = req_body.into();

        assert_eq!(expected, actual)
    }
}
