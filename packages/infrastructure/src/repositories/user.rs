use std::error::Error;

use anyhow::anyhow;
use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};

use application::{error::ApplicationError, interfaces::repositories::user::UserRepository};
use domain::user::{
    entities::user::User,
    value_objects::{
        email::Email, first_name::FirstName, last_name::LastName, password::Password,
        user_id::UserId,
    },
};

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: Pool<Postgres>,
}

impl PostgresUserRepository {
    pub async fn new(url: &str, max_connections: u32) -> Result<Self, Box<dyn Error>> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(url)
            .await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(&self, user: &User) -> Result<(), ApplicationError> {
        sqlx::query("INSERT INTO users VALUES ($1, $2, $3, $4, $5)")
            .bind(user.id().to_string())
            .bind(user.first_name().to_string())
            .bind(user.last_name().to_string())
            .bind(user.email().to_string())
            .bind(user.password().to_string())
            .execute(&self.pool)
            .await
            .map_err(|err| ApplicationError::UnexpectedError(anyhow!(err)))?;

        Ok(())
    }
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, ApplicationError> {
        let result = sqlx::query("SELECT * FROM users WHERE email = $1")
            .bind(email.to_string())
            .fetch_one(&self.pool)
            .await;

        let row = match result {
            Ok(row) => row,
            Err(err) => match err {
                sqlx::Error::RowNotFound => return Ok(None),
                _ => return Err(ApplicationError::UnexpectedError(anyhow!(err))),
            },
        };

        let id = row.get::<String, _>("id");
        let first_name = row.get::<String, _>("first_name");
        let last_name = row.get::<String, _>("last_name");
        let email = row.get::<String, _>("email");
        let hashed_password = row.get::<String, _>("password");

        let user = User::new(
            UserId::new(id),
            FirstName::new(first_name),
            LastName::new(last_name),
            Email::new(&email).unwrap(),
            Password::new_with(hashed_password),
        );

        Ok(Some(user))
    }
}
