use crate::models::message::MessageModel;
use anyhow::{Error, Result};
use async_graphql::{Object, Context};
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn find_by_user_id_and_time_range(
        &self,
        ctx: &Context<'_>,
        user_id: i32,
        start_time: Option<NaiveDateTime>,
        end_time: Option<NaiveDateTime>,
    ) -> Result<Vec<MessageModel>, Error> {
        let pool = ctx.data::<PgPool>().expect("Failed to get pool.");
        let rows =
            MessageModel::find_by_user_id_and_time_range(user_id, start_time, end_time, pool)
                .await?;
        Ok(rows)
    }
}

#[Object]
impl MessageModel {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn user_id(&self) -> i32 {
        self.user_id
    }

    async fn message(&self) -> String {
        self.message.clone()
    }

    async fn parent_id(&self) -> Option<i32> {
        self.parent_id
    }

    async fn created_at(&self) -> Option<NaiveDateTime> {
        self.created_at
    }

    async fn updated_at(&self) -> Option<NaiveDateTime> {
        self.updated_at
    }
}