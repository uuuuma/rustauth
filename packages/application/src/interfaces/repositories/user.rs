use std::error::Error;

use async_trait::async_trait;

use domain::user::{entities::user::User, value_objects::email::Email};

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: &User) -> Result<(), Box<dyn Error>>;
    async fn find_by_email(&self, email: &Email) -> Result<User, Box<dyn Error>>;
}
