use std::{error::Error, ops::Not};

use super::{query::SigninQuery, response::SigninResponse};
use crate::interfaces::{
    authentication::token_generator::TokenGenerator, repositories::user::UserRepository,
};
use anyhow::anyhow;
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
    pub async fn handle(&self, query: &SigninQuery) -> Result<SigninResponse, Box<dyn Error>> {
        let email = Email::new(query.email().clone());
        let user = self.user_repository.find_by_email(&email).await?;

        if user.password().is_same(query.password()).not() {
            return Err(anyhow!("password is invalid").into());
        }

        let token = self.token_generator.generate(user.id());

        Ok(SigninResponse::new(token))
    }
}
