use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::GraphQL;
use dotenvy::dotenv;
use gql::{mutations::MutationRoot, queries::QueryRoot};
use sqlx::PgPool;
use std::env;

mod gql;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().expect(".env file not found");

    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap_or("".to_string()))
        .await
        .expect("Failed to connect to Postgres.");
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .finish();
    let address = address();

    HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(GraphQL::new(schema.clone())),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind(address)?
    .run()
    .await
}

fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".into())
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}
