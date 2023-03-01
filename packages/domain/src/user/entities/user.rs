use std::fmt::Debug;

use crate::user::value_objects::{
    email::Email, first_name::FirstName, last_name::LastName, password::Password, user_id::UserId,
};

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
