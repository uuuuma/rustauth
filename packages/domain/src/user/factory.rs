use super::{
    entities::user::User,
    value_objects::{
        email::Email, first_name::FirstName, last_name::LastName, password::Password,
        user_id::UserId,
    },
};

pub struct UserFactory;

impl UserFactory {
    pub fn create(
        id: String,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
    ) -> User {
        User::new(
            UserId::new(id),
            FirstName::new(first_name),
            LastName::new(last_name),
            Email::new(email),
            Password::new(password),
        )
    }
}
