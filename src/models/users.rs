use anyhow::Error;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UsersModel {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UsersModelResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl UsersModel {
    pub async fn create(
        name: String,
        email: String,
        password: String,
        pool: &sqlx::PgPool,
    ) -> Result<UsersModelResponse, Error> {
        // Hash password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Unable to hash password.")
            .to_string();
        let parsed_hash = PasswordHash::new(&password_hash).expect("Unable to parse hash.");
        println!("parsed_hash: {}", parsed_hash);

        let row = sqlx::query_as!(
            UsersModel,
            r#"
            INSERT INTO users (name, email, password)
            VALUES ($1, $2, $3)
            RETURNING id, name, email, password, created_at, updated_at
            "#,
            name,
            email,
            password_hash
        )
        .fetch_one(pool)
        .await?;

        Ok(UsersModelResponse {
            id: row.id,
            name: row.name,
            email: row.email,
        })
    }
}
