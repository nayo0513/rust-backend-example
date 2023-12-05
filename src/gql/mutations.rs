use crate::models::{
    message::{MessageModel, MessageModelResponse},
    users::{UsersModel, UsersModelResponse},
};
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
    ) -> Result<MessageModelResponse, Error> {
        let pool = ctx.data::<PgPool>().expect("Failed to get pool.");
        let row = MessageModel::create(user_id, message, parent_id, pool).await?;
        Ok(row)
    }

    async fn modify_message(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
        message: String,
        token: String,
    ) -> Result<MessageModelResponse, Error> {
        let pool = ctx.data::<PgPool>().expect("Failed to get pool.");
        let row = MessageModel::modify(id, message, pool, token).await?;
        Ok(row)
    }

    async fn delete_message(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
        token: String,
    ) -> Result<i32, Error> {
        let pool = ctx.data::<PgPool>().expect("Failed to get pool.");
        let row = MessageModel::delete(id, pool, token).await?;
        Ok(row)
    }

    async fn create_user(
        &self,
        ctx: &async_graphql::Context<'_>,
        name: String,
        email: String,
        password: String,
    ) -> Result<UsersModelResponse, Error> {
        let pool = ctx.data::<PgPool>().expect("Failed to get pool.");
        let row = UsersModel::create(name, email, password, pool).await?;
        Ok(row)
    }
}
