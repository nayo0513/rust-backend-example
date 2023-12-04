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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use argon2::PasswordVerifier;
    use sqlx::{query_as, PgPool};

    #[sqlx::test]
    async fn create(pool: PgPool) -> Result<()> {
        let name = "test";
        let email = "example.example.com";
        let password = "password";

        UsersModel::create(
            name.to_string(),
            email.to_string(),
            password.to_string(),
            &pool,
        )
        .await?;
        let row = query_as!(
            UsersModel,
            r#"
            SELECT id, name, email, password, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_all(&pool)
        .await?;
        assert_eq!(row.len(), 1);

        // Check password
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&row[0].password).expect("Unable to parse hash.");
        assert!(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok());

        Ok(())
    }
}
