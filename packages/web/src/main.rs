use std::error::Error;

use application::authentication::commands::register::{
    command::RegisterCommand, handler::RegisterHandler,
};
use infrastructure::repositories::user::PostgresUserRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let user_repository =
        PostgresUserRepository::new("postgress://admin:admin@localhost/admin", 5).await?;
    let register_handler = RegisterHandler::new(user_repository);

    let register_command = RegisterCommand::new(
        "first_name".to_string(),
        "last_name".to_string(),
        "user@example.com".to_string(),
        "password".to_string(),
    );
    register_handler.handle(&register_command).await?;

    Ok(())
}
