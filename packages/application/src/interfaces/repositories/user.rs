use std::error::Error;

use async_trait::async_trait;

use domain::user::entities::user::User;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: &User) -> Result<(), Box<dyn Error>>;
}
