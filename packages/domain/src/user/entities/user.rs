use std::fmt::Debug;

use crate::user::value_objects::{
    email::Email, first_name::FirstName, last_name::LastName, password::Password, user_id::UserId,
};

#[derive(Eq)]
pub struct User {
    id: UserId,
    first_name: FirstName,
    last_name: LastName,
    email: Email,
    password: Password,
}

impl User {
    pub fn new(
        id: UserId,
        first_name: FirstName,
        last_name: LastName,
        email: Email,
        password: Password,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            email,
            password,
        }
    }
    pub fn id(&self) -> &UserId {
        &self.id
    }
    pub fn first_name(&self) -> &FirstName {
        &self.first_name
    }
    pub fn last_name(&self) -> &LastName {
        &self.last_name
    }
    pub fn email(&self) -> &Email {
        &self.email
    }
    pub fn password(&self) -> &Password {
        &self.password
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("first_name", &self.first_name)
            .field("last_name", &self.last_name)
            .field("email", &self.email)
            .field("password", &self.password)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_eq_works() {
        let user_id = "id";

        let user = User::new(
            UserId::new(user_id.to_string()),
            FirstName::new("first_name".to_string()),
            LastName::new("last_name".to_string()),
            Email::new("test@example.com".to_string()),
            Password::new(&"password".to_string()),
        );

        let other = User::new(
            UserId::new(user_id.to_string()),
            FirstName::new("other_first_name".to_string()),
            LastName::new("other_last_name".to_string()),
            Email::new("other_test@example.com".to_string()),
            Password::new(&"other_password".to_string()),
        );

        assert_eq!(user, other)
    }
}
