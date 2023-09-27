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

#[tokio::main]
async fn main() {
    let state = CarnyState::new().await;

    let app: Router = Router::new()
        .route("/", get(root))
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .with_state(state.clone());

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello world"
}



// async fn register(Json(payload): Json<RegisterInput>) -> String {
//     if payload.username.is_empty() || payload.password.is_empty() {
//         return "Username or password not provided.".to_string();
//     }
//     let db = SqlitePool::connect("sqlite://sqlite.db").await.unwrap();
//     let hashed_password = hash_password(&payload.password, b"sosecurewow", 12).unwrap();
//     // let is_same_passwd: bool = verify_password("1test123", a.as_str(), b"sosecurewow").unwrap();
//
//     let result = sqlx::query("INSERT INTO users (name, password) VALUES ($1, $2)")
//         .bind(payload.username)
//         .bind(hashed_password)
//         .execute(&db).await;
//     "Done".to_string()
// }
