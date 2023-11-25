use crate::models::message::MessageModel;
use anyhow::{Error, Result};
use async_graphql::Object;
use sqlx::postgres::PgPool;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_message(
        &self,
        ctx: &async_graphql::Context<'_>,
        user_id: i32,
        message: String,
        parent_id: Option<i32>,
    ) -> Result<MessageModel, Error> {
        let pool = ctx.data::<PgPool>().expect("Failed to get pool.");
        let row = MessageModel::create(user_id, message, parent_id, pool).await?;
        Ok(row)
    }

    async fn modify_message(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
        message: String,
    ) -> Result<MessageModel, Error> {
        let pool = ctx.data::<PgPool>().expect("Failed to get pool.");
        let row = MessageModel::modify(id, message, pool).await?;
        Ok(row)
    }

    async fn delete_message(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
    ) -> Result<i32, Error> {
        let pool = ctx.data::<PgPool>().expect("Failed to get pool.");
        let row = MessageModel::delete(id, pool).await?;
        Ok(row)
    }
}