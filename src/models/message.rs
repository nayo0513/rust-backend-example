use anyhow::{Error, Result};
use chrono::{DateTime, Utc};
use sqlx::{query, query_as, FromRow, PgPool};

#[derive(Debug, FromRow, serde::Deserialize, serde::Serialize)]
pub struct MessageModel {
    pub id: i32,
    pub user_id: i32,
    pub message: String,
    pub parent_id: Option<i32>,
    #[serde(rename = "messageTime")]
    pub message_time: DateTime<Utc>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
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
            insert into message (user_id, message, parent_id, message_time)
            values ($1, $2, $3, now())
            returning id, user_id, message, parent_id, message_time, created_at, updated_at
            "#,
            user_id,
            message,
            parent_id,
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
            set message = $1, updated_at = now()
            where id = $2
            returning id, user_id, message, parent_id, message_time, created_at, updated_at
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
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
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

        let rows = if start_time.is_some() && end_time.is_some() {
            // If both start_time and end_time are specified.
            query_as!(
                MessageModel,
                r#"
                select id, user_id, message, parent_id, message_time, created_at, updated_at
                from message
                where user_id = $1
                and message_time between $2 and $3
                "#,
                user_id,
                start_time,
                end_time
            )
            .fetch_all(pool)
            .await?
        } else if start_time.is_some() {
            // If only start_time is specified.
            query_as!(
                MessageModel,
                r#"
                select id, user_id, message, parent_id, message_time, created_at, updated_at
                from message
                where user_id = $1
                and message_time >= $2
                "#,
                user_id,
                start_time
            )
            .fetch_all(pool)
            .await?
        } else if end_time.is_some() {
            // If only end_time is specified.
            query_as!(
                MessageModel,
                r#"
                select id, user_id, message, parent_id, message_time, created_at, updated_at
                from message
                where user_id = $1
                and message_time <= $2
                "#,
                user_id,
                end_time
            )
            .fetch_all(pool)
            .await?
        } else {
            // If both start_time and end_time are not specified.
            query_as!(
                MessageModel,
                r#"
                select id, user_id, message, parent_id, message_time, created_at, updated_at
                from message
                where user_id = $1
                "#,
                user_id
            )
            .fetch_all(pool)
            .await?
        };

        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::message::MessageModel;
    use anyhow::Result;
    use sqlx::{query, query_as, PgPool};

    #[sqlx::test]
    async fn create(pool: PgPool) -> Result<()> {
        let user_id = 1;
        let message = "test message".to_string();
        let parent_id = None;

        // Create user.
        query!(
            r#"
            insert into users (id, name, password, email)
            values ($1, $2, $3, $4)
            "#,
            user_id,
            "test",
            "test",
            "test@example.com"
        )
        .execute(&pool)
        .await?;
        // Create message.
        let row = MessageModel::create(user_id, message.clone(), parent_id, &pool).await?;

        assert_eq!(row.user_id, user_id);
        assert_eq!(row.message, message);
        assert_eq!(row.parent_id, parent_id);

        Ok(())
    }

    #[sqlx::test]
    async fn modify(pool: PgPool) -> Result<()> {
        let user_id = 1;
        let message = "test message".to_string();

        // Create user.
        query!(
            r#"
            insert into users (id, name, password, email)
            values ($1, $2, $3, $4)
            "#,
            user_id,
            "test",
            "test",
            "test@example.com"
        )
        .execute(&pool)
        .await?;
        // Create message.
        query!(
            r#"
            insert into message (user_id, message, message_time)
            values ($1, $2, now())
            "#,
            user_id,
            message.clone(),
        )
        .execute(&pool)
        .await?;

        let modified_message = "modified message".to_string();
        // Modify message.
        let row = MessageModel::modify(1, modified_message.clone(), &pool).await?;

        assert_eq!(row.message, modified_message);
        Ok(())
    }

    #[sqlx::test]
    async fn delete(pool: PgPool) -> Result<()> {
        let user_id = 1;
        let message = "test message".to_string();

        // Create user.
        query!(
            r#"
            insert into users (id, name, password, email)
            values ($1, $2, $3, $4)
            "#,
            user_id,
            "test",
            "test",
            "test@example.com"
        )
        .execute(&pool)
        .await?;
        // Create message.
        query!(
            r#"
            insert into message (user_id, message, message_time)
            values ($1, $2, now())
            "#,
            user_id,
            message.clone(),
        )
        .execute(&pool)
        .await?;

        // Delete message.
        MessageModel::delete(user_id, &pool).await?;

        // Check if message deleted.
        let row = query_as!(
            MessageModel,
            r#"
            select *
            from message
            "#
        )
        .fetch_all(&pool)
        .await?;
        assert_eq!(row.len(), 0);

        Ok(())
    }

    #[sqlx::test]
    async fn find_by_user_id_and_time_range(pool: PgPool) -> Result<()> {
        let user_id = 1;
        let message = "test message".to_string();

        // Create user.
        query!(
            r#"
            insert into users (id, name, password, email)
            values ($1, $2, $3, $4)
            "#,
            user_id,
            "test",
            "test",
            "test@example.com"
        )
        .execute(&pool)
        .await?;
        // Create message.
        let message_time = chrono::Utc::now();
        query!(
            r#"
            insert into message (user_id, message, message_time)
            values ($1, $2, $3)
            "#,
            user_id,
            message.clone(),
            message_time
        )
        .execute(&pool)
        .await?;

        // Find message.
        // Find by user_id.
        let rows = MessageModel::find_by_user_id_and_time_range(user_id, None, None, &pool).await?;
        assert_eq!(rows.len(), 1);
        // Find by user_id and start_time.
        let rows =
            MessageModel::find_by_user_id_and_time_range(user_id, Some(message_time), None, &pool)
                .await?;
        assert_eq!(rows.len(), 1);
        // Find by user_id and end_time.
        let rows =
            MessageModel::find_by_user_id_and_time_range(user_id, None, Some(message_time), &pool)
                .await?;
        assert_eq!(rows.len(), 1);
        // Find by user_id and start_time and end_time.
        let rows = MessageModel::find_by_user_id_and_time_range(
            user_id,
            Some(message_time),
            Some(message_time),
            &pool,
        )
        .await?;
        assert_eq!(rows.len(), 1);
        Ok(())
    }
}
