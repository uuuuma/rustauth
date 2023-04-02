use std::error::Error;

use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use application::interfaces::repositories::user::UserRepository;
use domain::user::entities::user::User;

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
    async fn save(&self, user: &User) -> Result<(), Box<dyn Error>> {
        sqlx::query("INSERT INTO users VALUES ($1, $2, $3, $4, $5)")
            .bind(user.id().to_string())
            .bind(user.first_name().to_string())
            .bind(user.last_name().to_string())
            .bind(user.email().to_string())
            .bind(user.password().to_string())
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
