use std::error::Error;

use super::query::SigninQuery;
use crate::interfaces::repositories::user::UserRepository;
use domain::user::{entities::user::User, value_objects::email::Email};

pub struct SigninHandler<R>
where
    R: UserRepository,
{
    user_repository: R,
}

impl<R> SigninHandler<R>
where
    R: UserRepository,
{
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }
    pub async fn handle(&self, query: &SigninQuery) -> Result<User, Box<dyn Error>> {
        let email = Email::new(query.email().clone());
        self.user_repository.load_by_email(&email).await
    }
}
