use std::net::SocketAddr;

#[macro_use]
extern crate dotenv_codegen;

use axum::{
    routing::{get, post},
    Router,
    Server
};
use axum_client_ip::{
    InsecureClientIp,
    SecureClientIp,
    SecureClientIpSource
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

#[derive(Clone)]
pub struct CarnyState {
    pool: SqlitePool
}

impl CarnyState {
    pub async fn new() -> Self {
        match Sqlite::create_database("sqlite://sqlite.db").await {
            Ok(_) => println!("Ok"),
            Err(e) => panic!("Error -> {}", e)
        }
        let pool = SqlitePool::connect("sqlite://sqlite.db").await.unwrap();
        let result = sqlx::query(&tables::CREATE_USERS).execute(&pool).await.unwrap();
        println!("User table creation -> {:?}", result);

        Self { pool }
    }
}

async fn handler(insecure_ip: InsecureClientIp, secure_ip: SecureClientIp) -> String {
    format!("{insecure_ip:?} {secure_ip:?}")
}

#[tokio::main]
async fn main() {
    let state = CarnyState::new().await;

    let app: Router = Router::new()
        .route("/", get(root))
        .route("/iptest", get(handler))
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
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
