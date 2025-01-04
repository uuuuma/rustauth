use std::error::Error;

use fake::{faker::internet::ja_jp as fake_internet, faker::name::ja_jp as fake_name, Fake};

use application::authentication::{
    commands::signup::{command::SignupCommand, handler::SignupHandler},
    queries::signin::{handler::SigninHandler, query::SigninQuery},
};
use infrastructure::{
    authentication::{jwt_generator::JwtGenerator, uuid_generator::UuidGenerator},
    repositories::user::PostgresUserRepository,
    services::chrono_datetime_service::ChronoDateTimeService,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let user_repository =
        PostgresUserRepository::new("postgres://admin:admin@localhost/admin", 5).await?;
    let id_generator = UuidGenerator::new();
    let datetime_service = ChronoDateTimeService::new();
    let jwt_generator = JwtGenerator::new(id_generator.clone(), datetime_service);

    let signup_handler = SignupHandler::new(user_repository.clone(), id_generator.clone());
    let signin_handler = SigninHandler::new(user_repository.clone(), jwt_generator);

    let email = fake_internet::FreeEmail()
        .fake::<String>()
        .split(' ')
        .collect::<Vec<_>>()
        .join("_");

    let signup_command = SignupCommand::new(
        fake_name::FirstName().fake::<String>(),
        fake_name::LastName().fake::<String>(),
        email.clone(),
        "Password1234".to_string(),
    );
    signup_handler.handle(&signup_command).await?;

    let signin_query = SigninQuery::new(email, "Password1234".to_string());
    let signin_response = signin_handler.handle(&signin_query).await?;

    println!("{:#?}", signin_response);

    Ok(())
}
