use std::error::Error;

use application::authentication::commands::signup::{
    command::SignupCommand, handler::SignupHandler,
};
use infrastructure::{
    authentication::ulid_generator::UlidGenerator, repositories::user::PostgresUserRepository,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let user_repository =
        PostgresUserRepository::new("postgres://admin:admin@localhost/admin", 5).await?;
    let id_generator = UlidGenerator::new();
    let signup_handler = SignupHandler::new(user_repository, id_generator);

    let signup_command = SignupCommand::new(
        "first_name".to_string(),
        "last_name".to_string(),
        "user@example.com".to_string(),
        "password".to_string(),
    );
    signup_handler.handle(&signup_command).await?;

    Ok(())
}
