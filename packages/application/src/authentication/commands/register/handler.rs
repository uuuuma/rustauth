use std::error::Error;

use super::command::RegisterCommand;

use domain::user::factory::UserFactory;
use infrastructure::repositories::user::PostgresUserRepository;

pub struct RegisterHandler {
    user_repository: PostgresUserRepository,
}

impl RegisterHandler {
    pub fn new(user_repository: PostgresUserRepository) -> Self {
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
