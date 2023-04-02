use std::error::Error;

use super::command::RegisterCommand;
use crate::interfaces::repositories::user::UserRepository;
use domain::user::factory::UserFactory;

pub struct RegisterHandler<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> RegisterHandler<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }
    pub async fn handle(&self, command: &RegisterCommand) -> Result<(), Box<dyn Error>> {
        let user = UserFactory::create(
            // TODO: generate random id
            "id".to_string(),
            command.first_name().clone(),
            command.last_name().clone(),
            command.email().clone(),
            command.password().clone(),
        );

        self.user_repository.save(&user).await?;

        Ok(())
    }
}
