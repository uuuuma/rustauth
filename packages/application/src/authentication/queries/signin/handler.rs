use std::ops::Not;

use super::{query::SigninQuery, response::SigninResponse};
use crate::{
    error::ApplicationError,
    interfaces::{
        authentication::token_generator::TokenGenerator, repositories::user::UserRepository,
    },
};
use domain::user::value_objects::email::Email;

pub struct SigninHandler<R, T>
where
    R: UserRepository,
    T: TokenGenerator,
{
    user_repository: R,
    token_generator: T,
}

impl<R, T> SigninHandler<R, T>
where
    R: UserRepository,
    T: TokenGenerator,
{
    pub fn new(user_repository: R, token_generator: T) -> Self {
        Self {
            user_repository,
            token_generator,
        }
    }
    pub async fn handle(&self, query: &SigninQuery) -> Result<SigninResponse, ApplicationError> {
        let email = Email::new(query.email()).unwrap();
        let user = match self.user_repository.find_by_email(&email).await? {
            Some(user) => user,
            None => return Err(ApplicationError::UserIsNotFoundError),
        };

        if user.password().is_same(query.password()).not() {
            return Err(ApplicationError::InvalidParameters {
                description: "email or password is wrong",
            });
        }

        let token = self.token_generator.generate(user.id());

        Ok(SigninResponse::new(token))
    }
}
