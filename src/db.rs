use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use crate::models::User;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("SQLx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Clone)]
pub struct Db {
    pool: MySqlPool,
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self, DbError> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        Ok(Self { pool })
    }

    pub async fn create_user(&self, user: &User) -> Result<i64, DbError> {
        let id = sqlx::query!(
            "INSERT INTO users (username, email) VALUES (?, ?)",
            user.username,
            user.email
        )
        .execute(&self.pool)
        .await?
        .last_insert_id();
        Ok(id as i64)
    }

    pub async fn get_user(&self, id: i64) -> Result<Option<User>, DbError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, username, email FROM users WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }
}

