use async_trait::async_trait;

use domain::user::{entities::user::User, value_objects::email::Email};

use crate::error::ApplicationError;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: &User) -> Result<(), ApplicationError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, ApplicationError>;
}
