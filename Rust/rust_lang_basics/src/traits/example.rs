use std::default;

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub enum Role {
    Admin,
    Standard,
    #[default]
    Guest,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct User {
    id: u32,
    name: String,
    role: Role,
    #[serde(skip)]
    pet_name: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn creating_user() {
        let user: User = User {
            id: 32,
            name: "nitih".to_owned(),
            role: Role::Admin,
            pet_name: None,
        };
        // need of Debug
        print!("{:?}", user);
    }
    #[test]
    fn cloning_user() {
        let user: User = User {
            id: 32,
            name: "nitih".to_owned(),
            role: Role::Admin,
            pet_name: None,
        };
        // use clone
        let user2 = user.clone();
    }
    #[test]
    fn default() {
        let user = User::default();
        print!("{:?}", user)
    }

    #[test]
    fn equation() {
        let user1 = User::default();
        let user2 = User::default();
        assert_eq!(user1, user2);
    }
    #[test]
    fn serialize() {
        let user = User {
            id: 343,
            name: "niothin".to_owned(),
            role: Role::Admin,
            pet_name: Some("nithin".to_owned()),
        };
        let user_json = serde_json::to_string(&user).unwrap();

        print!("{:?}", user_json);
    }
    #[test]
    fn deserialize() {
        let str = "{\"id\":343,\"name\":\"niothin\",\"role\":\"Admin\"}";
        let json_user: User = serde_json::from_str(&str).unwrap();
        print!("{:?}", json_user);
    }
}
