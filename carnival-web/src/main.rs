#[macro_use]
extern crate dotenv_codegen;

use std::{net::SocketAddr, env};
use http::{Method, HeaderName};
use rendering::components::{register_form, login_form, login_route, register_route};
use tower::{ServiceBuilder, ServiceExt, Service};
use tower_http::cors::{Any, CorsLayer};
use axum::{
    routing::{get, post},
    Router,
    Server, http::header::CONTENT_TYPE
};
use api::endpoints::{
    register,
    login
};
use sqlx::{SqlitePool, Sqlite, migrate::MigrateDatabase};
use crate::db::queries::tables;

mod db;
mod api;
mod rendering;


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
    if cfg!(debug_assertions) {
        env::set_var("RUST_BACKTRACE", "1");
    }
    let state = CarnyState::new().await;
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([
           CONTENT_TYPE,
           HeaderName::from_lowercase(b"hx-request").unwrap(),
           HeaderName::from_lowercase(b"hx-current-url").unwrap(),
        ]);

    let app: Router = Router::new()
        .route("/", get(root))
        .route("/login", get(login_route))
        .route("/register", get(register_route))
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .route("/components/registration", get(register_form))
        .route("/components/login", get(login_form))
        .layer(cors)
        .with_state(state.clone());

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello world"
}
