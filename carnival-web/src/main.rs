#[macro_use]
extern crate dotenv_codegen;

use std::{net::SocketAddr, env};
use http::{Method, HeaderName};
use rendering::components::{register_form, login_form, hero, queue_table, queue_user_panel};
use rendering::routes::{register_route, login_route, index, queue_route, leaderboard_route, profile_route};
use tower_http::cors::{Any, CorsLayer};
use axum::{
    routing::{get, post},
    Router,
    Server, http::header::CONTENT_TYPE
};
use api::endpoints::{
    register,
    login, join_queue, leave_queue
};
use sqlx::{SqlitePool, Sqlite, migrate::MigrateDatabase};
use crate::db::queries::tables;
use crate::db::services::overwatch_match::ResolvedOverwatchMatch;

mod db;
mod api;
mod rendering;


const HMAC_KEY: &[u8] = dotenv!("HMAC_KEY").as_bytes();
const DATABASE_URL: &str = dotenv!("DATABASE_URL");

#[derive(Clone)]
pub struct CarnyState {
    pool: SqlitePool
}

impl CarnyState {
    pub async fn new() -> Self {
        match Sqlite::create_database(DATABASE_URL).await {
            Ok(_) => println!("Ok"),
            Err(e) => panic!("Error -> {}", e)
        }

        let pool = SqlitePool::connect(DATABASE_URL).await.unwrap();
        // Probably move these to create_tables or something at some point.
        let create_user_table_result = sqlx::query(tables::CREATE_USERS).execute(&pool).await.unwrap();

        // Probably move these to create_tables or something at some point.
        let create_user_table_result = sqlx::query(&tables::CREATE_USERS).execute(&pool).await.unwrap();
        println!("User table creation -> {:?}", create_user_table_result);

        let create_session_result = sqlx::query(tables::CREATE_SESSION_TOKENS).execute(&pool).await.unwrap();
        println!("Session Token table creation -> {:?}", create_session_result);

        let create_ow_map_result = sqlx::query(&tables::CREATE_OW_MAP).execute(&pool).await.unwrap();
        println!("Overwatch Maps table creation -> {:?}", create_ow_map_result);

        let create_ow_match_player_result = sqlx::query(&tables::CREATE_OW_MATCH_THRU).execute(&pool).await.unwrap();
        println!("Overwatch Match thru table creation -> {:?}", create_ow_match_player_result);

        let create_ow_match_result = sqlx::query(&tables::CREATE_OW_MATCH).execute(&pool).await.unwrap();
        println!("Overwatch Match table creation -> {:?}", create_ow_match_result);

        let create_queue_result = sqlx::query(&tables::CREATE_QUEUE).execute(&pool).await.unwrap();
        println!("Queue table creation -> {:?}", create_queue_result);

        let create_queued_players_result = sqlx::query(&tables::CREATE_QUEUED_PLAYERS).execute(&pool).await.unwrap();
        println!("Queued Players table creation -> {:?}", create_queued_players_result);

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
           HeaderName::from_lowercase(b"hx-target").unwrap(),
        ]);

    let app: Router = Router::new()
        // User-facing
        .route("/", get(index))
        .route("/login", get(login_route))
        .route("/register", get(register_route))
        .route("/queue", get(queue_route))
        .route("/leaderboards", get(leaderboard_route))
        .route("/@:username", get(profile_route))
        // Components
        .route("/components/registration", get(register_form))
        .route("/components/login", get(login_form))
        .route("/components/hero", get(hero))
        .route("/components/queue_table/:username", get(queue_table))
        .route("/components/queue_user_table/:username", get(queue_user_panel))
        // Endpoints
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .route("/api/join_queue", post(join_queue))
        .route("/api/leave_queue", post(leave_queue))
        .layer(cors)
        .with_state(state.clone());

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
