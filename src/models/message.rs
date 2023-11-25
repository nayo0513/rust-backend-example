use anyhow::{Error, Result};
use chrono::NaiveDateTime;
use sqlx::{query, query_as, FromRow, PgPool};

#[derive(Debug, FromRow, serde::Deserialize, serde::Serialize)]
pub struct MessageModel {
    pub id: i32,
    pub user_id: i32,
    pub message: String,
    pub parent_id: Option<i32>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
}

impl MessageModel {
    pub async fn create(
        user_id: i32,
        message: String,
        parent_id: Option<i32>,
        pool: &PgPool,
    ) -> Result<MessageModel, Error> {
        // Check if user_id valid.
        if query!(
            r#"
            select id
            from users
            where id = $1
            "#,
            user_id
        )
        .fetch_one(pool)
        .await
        .is_err()
        {
            return Err(Error::msg("User not found."));
        }

        // Check if parent_id exists.
        if parent_id.is_some()
            && query!(
                r#"
            select id
            from message
            where id = $1
            "#,
                parent_id
            )
            .fetch_one(pool)
            .await
            .is_err()
        {
            return Err(Error::msg("Parent message not found."));
        }

        let row = query_as!(
            MessageModel,
            r#"
            insert into message (user_id, message, parent_id)
            values ($1, $2, $3)
            returning id, user_id, message, parent_id, created_at, updated_at
            "#,
            user_id,
            message,
            parent_id
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    pub async fn modify(id: i32, message: String, pool: &PgPool) -> Result<MessageModel, Error> {
        // Check if message exists.
        if query!(
            r#"
            select id
            from message
            where id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .is_err()
        {
            return Err(Error::msg("Message not found."));
        }

        let row = query_as!(
            MessageModel,
            r#"
            update message
            set message = $1
            where id = $2
            returning id, user_id, message, parent_id, created_at, updated_at
            "#,
            message,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<i32, Error> {
        // Check if message exists.
        if query!(
            r#"
            select id
            from message
            where id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .is_err()
        {
            return Err(Error::msg("Message not found."));
        }

        let row = query!(
            r#"
            delete from message
            where id = $1
            returning id
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(row.id)
    }

    pub async fn find_by_user_id_and_time_range(
        user_id: i32,
        start_time: Option<NaiveDateTime>,
        end_time: Option<NaiveDateTime>,
        pool: &PgPool,
    ) -> Result<Vec<MessageModel>, Error> {
        // Check if user_id valid.
        if query!(
            r#"
            select id
            from users
            where id = $1
            "#,
            user_id
        )
        .fetch_one(pool)
        .await
        .is_err()
        {
            return Err(Error::msg("User not found."));
        }

        let rows = query_as!(
            MessageModel,
            r#"
            select id, user_id, message, parent_id, created_at, updated_at
            from message
            where user_id = $1
            and created_at between $2 and $3
            "#,
            user_id,
            start_time,
            end_time
        )
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }
}
