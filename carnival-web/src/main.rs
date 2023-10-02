#[macro_use]
extern crate dotenv_codegen;

use std::ops::ControlFlow;
use std::{net::SocketAddr, env};
use axum::TypedHeader;
use axum::extract::ConnectInfo;
use axum::extract::ws::Message;
use axum::response::IntoResponse;
use http::{Method, HeaderName};
use rendering::components::{register_form, login_form, hero, queue_table, queue_user_panel, leaderboard_comp};
use rendering::routes::{register_route, login_route, index, queue_route, leaderboard_route, profile_route};
use tower_http::cors::{Any, CorsLayer};
use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade}, State},
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

mod db;
mod api;
mod rendering;


const HMAC_KEY: &[u8] = dotenv!("HMAC_KEY").as_bytes();
const DATABASE_URL: &str = dotenv!("DATABASE_URL");
const DOMAIN: &str = dotenv!("DOMAIN");

#[derive(Clone)]
pub struct CarnyState {
    pool: SqlitePool
}

async fn create_tables(pool: &SqlitePool) {

    let create_user_table_result = sqlx::query(tables::CREATE_USERS).execute(pool).await.unwrap();
    println!("User table creation -> {:?}", create_user_table_result);

    let create_session_result = sqlx::query(tables::CREATE_SESSION_TOKENS).execute(pool).await.unwrap();
    println!("Session Token table creation -> {:?}", create_session_result);

    let create_ow_map_result = sqlx::query(&tables::CREATE_OW_MAP).execute(pool).await.unwrap();
    println!("Overwatch Maps table creation -> {:?}", create_ow_map_result);

    let create_ow_match_player_result = sqlx::query(&tables::CREATE_OW_MATCH_THRU).execute(pool).await.unwrap();
    println!("Overwatch Match thru table creation -> {:?}", create_ow_match_player_result);

    let create_ow_match_result = sqlx::query(&tables::CREATE_OW_MATCH).execute(pool).await.unwrap();
    println!("Overwatch Match table creation -> {:?}", create_ow_match_result);

    let create_queue_result = sqlx::query(&tables::CREATE_QUEUE).execute(pool).await.unwrap();
    println!("Queue table creation -> {:?}", create_queue_result);

    let create_queued_players_result = sqlx::query(&tables::CREATE_QUEUED_PLAYERS).execute(pool).await.unwrap();
    println!("Queued Players table creation -> {:?}", create_queued_players_result);
}

impl CarnyState {
    pub async fn new() -> Self {
        match Sqlite::create_database(DATABASE_URL).await {
            Ok(_) => println!("Ok"),
            Err(e) => panic!("Error -> {}", e)
        }

        let pool = SqlitePool::connect(DATABASE_URL).await.unwrap();
        create_tables(&pool).await;

        Self { pool }
    }
}

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        env::set_var("RUST_BACKTRACE", "1");
    }

    //
    // Websocket shit 
    // Spawning a thread for websocket tcp listener.
    // Pretty sure this is a bad idea.
    // Concurrency issues with the db and that.
    //
    // thread::spawn(move || async move {
    //     let ws_router = Router::new()
    //         .route("/ws/notifications", get(ws_handler));
    //     Server::bind(&"0.0.0.0:6969".parse().unwrap())
    //         .serve(ws_router.into_make_service_with_connect_info::<SocketAddr>())
    //         .await
    //         .unwrap();
    // });
    //

    //
    // http routes
    //
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
        .route("/leaderboard", get(leaderboard_route))
        .route("/@:username", get(profile_route))
        // Components
        .route("/components/registration", get(register_form))
        .route("/components/login", get(login_form))
        .route("/components/hero", get(hero))
        .route("/components/leaderboard", get(leaderboard_comp))
        .route("/components/queue_table", get(queue_table))
        .route("/components/queue_user_table/:username", get(queue_user_panel))
        // Endpoints
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .route("/api/join_queue", post(join_queue))
        .route("/api/leave_queue", post(leave_queue))
        // Websockets..?
        // .route("/ws/notifications", get(ws_handler))
        .layer(cors)
        .with_state(state.clone());

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

// async fn ws_handler(
//     ws: WebSocketUpgrade,
//     // Don't think I give a shit about user-agent in the context of websockets.
//     user_agent: Option<TypedHeader<headers::UserAgent>>,
//     State(state): State<CarnyState>,
//     ConnectInfo(addr): ConnectInfo<SocketAddr>,
// ) -> impl IntoResponse {
//
//     ws.on_upgrade(move |socket| client_handler(socket, addr, state.pool))
// }
//
// async fn ws_send(
//     mut socket: WebSocket,
//     message: &str
// ) -> Result<WebSocket, axum::Error> {
//
//     // Feel like cloning socket might be a bad idea?
//     match socket.send(Message::Text(message.to_string())).await {
//         Ok(_)  => return Ok(socket),
//         Err(e) => return Err(e)
//     }
// }
//
// async fn client_handler(
//     mut socket: WebSocket,
//     client: SocketAddr,
//     pool: SqlitePool
// ) {
//     if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
//         println!("ping good -> {client}");
//         socket = ws_send(socket, "oqijweqiowej").await.unwrap();
//     } else {
//         println!("could not ping {client}");
//         return;
//     }
//
//     // socket.recv is surely blocking, no?
//     // For just this client I'd imagine.
//     if let Some(msg) = socket.recv().await {
//         if let Ok(msg) = msg {
//             while process_message(&msg, client).is_continue() {
//                 println!("{:#?}", msg);
//             }
//         } else {
//             println!("no recv");
//             return;
//         }
//     }
// }
//
// fn process_message(msg: &Message, client: SocketAddr) -> ControlFlow<(), ()> {
//     match msg {
//         Message::Text(t) => {
//             println!("{client}: {t:?}");
//         }
//         Message::Binary(d) => {
//             println!("{client}: {:?} ({})", d, d.len());
//         }
//         Message::Close(c) => {
//             if let Some(cf) = c {
//                 println!("{client} close with reason {:#?}", cf.reason);
//                 return ControlFlow::Break(());
//             }
//         }
//         Message::Pong(v) => {
//             println!("{client} pong {v:?}");
//         }
//         Message::Ping(v) => {
//             println!("{client} ping {v:#?}")
//         }
//     }
//
//     ControlFlow::Continue(())
// }
