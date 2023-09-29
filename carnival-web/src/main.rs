use std::net::SocketAddr;

#[macro_use]
extern crate dotenv_codegen;

use axum::{
    routing::{get, post},
    Router,
    Server
};
use api::endpoints::{
    register,
    login
};
use sqlx::{SqlitePool, Sqlite, migrate::MigrateDatabase};
use crate::db::queries::tables;

mod db;
mod api;


const HMAC_KEY: &[u8] = dotenv!("HMAC_KEY").as_bytes();
const SQLITE_DB_NAME: &str = dotenv!("SQLITE_DB_NAME");

#[derive(Clone)]
pub struct CarnyState {
    pool: SqlitePool
}

impl CarnyState {
    pub async fn new() -> Self {
        match Sqlite::create_database(SQLITE_DB_NAME).await {
            Ok(_) => println!("Ok"),
            Err(e) => panic!("Error -> {}", e)
        }
        let pool = SqlitePool::connect(SQLITE_DB_NAME).await.unwrap();
        let create_user_table_result = sqlx::query(&tables::CREATE_USERS).execute(&pool).await.unwrap();
        println!("User table creation -> {:?}", create_user_table_result);

        let create_session_result = sqlx::query(&tables::CREATE_SESSION_TOKENS).execute(&pool).await.unwrap();
        println!("Session Token table creation -> {:?}", create_session_result);

        Self { pool }
    }
}


#[tokio::main]
async fn main() {
    let state = CarnyState::new().await;

    let app: Router = Router::new()
        .route("/", get(root))
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .with_state(state.clone());

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello world"
}
