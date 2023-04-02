use std::error::Error;

use super::command::RegisterCommand;
use crate::interfaces::{
    authentication::id_generator::IdGenerator, repositories::user::UserRepository,
};
use domain::user::factory::UserFactory;

pub struct RegisterHandler<R, G>
where
    R: UserRepository,
    G: IdGenerator,
{
    user_repository: R,
    id_generator: G,
}

impl<R, G> RegisterHandler<R, G>
where
    R: UserRepository,
    G: IdGenerator,
{
    pub fn new(user_repository: R, id_generator: G) -> Self {
        Self {
            user_repository,
            id_generator,
        }
    }
    pub async fn handle(&self, command: &RegisterCommand) -> Result<(), Box<dyn Error>> {
        let id = self.id_generator.generate();

        let user = UserFactory::create(
            id,
            command.first_name().clone(),
            command.last_name().clone(),
            command.email().clone(),
            command.password().clone(),
        );

        self.user_repository.save(&user).await?;

        Ok(())
    }
}
