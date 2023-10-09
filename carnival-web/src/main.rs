#[macro_use]
extern crate dotenv_codegen;

use crate::db::queries::tables;
use api::endpoints::{
    forgot_password, join_queue, leave_queue, login, register, reset_password, save_settings,
};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    http::header::CONTENT_TYPE,
    response::IntoResponse,
    routing::{get, post},
    Router, Server,
};
use db::services::overwatch_match::{self, ResolvedOverwatchMatch};
use futures::{stream::StreamExt, SinkExt};
use http::{HeaderName, Method};
use rendering::routes::{
    index, leaderboard_route, login_route, profile_route, queue_route, register_route,
};
use rendering::{
    components::{
        hero, leaderboard_comp, login_form, profile_comp, queue_table, queue_user_panel,
        register_form, settings_user,
    },
    routes::settings_route,
};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::{env, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};

mod api;
mod commserver;
mod db;
mod rendering;

const HMAC_KEY: &[u8] = dotenv!("HMAC_KEY").as_bytes();
const DATABASE_URL: &str = dotenv!("DATABASE_URL");
const DOMAIN: &str = dotenv!("DOMAIN");

async fn create_tables(pool: &SqlitePool) {
    let create_user_table_result = sqlx::query(tables::CREATE_USERS)
        .execute(pool)
        .await
        .unwrap();
    println!("User table creation -> {:?}", create_user_table_result);

    let create_session_result = sqlx::query(tables::CREATE_SESSION_TOKENS)
        .execute(pool)
        .await
        .unwrap();
    println!(
        "Session Token table creation -> {:?}",
        create_session_result
    );

    let create_ow_map_result = sqlx::query(&tables::CREATE_OW_MAP)
        .execute(pool)
        .await
        .unwrap();
    println!(
        "Overwatch Maps table creation -> {:?}",
        create_ow_map_result
    );

    let create_ow_match_player_result = sqlx::query(&tables::CREATE_OW_MATCH_THRU)
        .execute(pool)
        .await
        .unwrap();
    println!(
        "Overwatch Match thru table creation -> {:?}",
        create_ow_match_player_result
    );

    let create_ow_match_result = sqlx::query(&tables::CREATE_OW_MATCH)
        .execute(pool)
        .await
        .unwrap();
    println!(
        "Overwatch Match table creation -> {:?}",
        create_ow_match_result
    );

    let create_queue_result = sqlx::query(&tables::CREATE_QUEUE)
        .execute(pool)
        .await
        .unwrap();
    println!("Queue table creation -> {:?}", create_queue_result);

    let create_queued_players_result = sqlx::query(&tables::CREATE_QUEUED_PLAYERS)
        .execute(pool)
        .await
        .unwrap();
    println!(
        "Queued Players table creation -> {:?}",
        create_queued_players_result
    );

    let create_password_reset_token_result = sqlx::query(&tables::CREATE_QUEUED_PLAYERS)
        .execute(pool)
        .await
        .unwrap();
    println!(
        "Password Reset Token table creation -> {:?}",
        create_password_reset_token_result
    );
}

#[derive(Clone)]
pub struct CarnyState {
    pool: SqlitePool,
}

impl CarnyState {
    pub async fn new() -> Self {
        match Sqlite::create_database(DATABASE_URL).await {
            Ok(_) => println!("Ok"),
            Err(e) => panic!("Error -> {}", e),
        }

        let pool = SqlitePool::connect(DATABASE_URL).await.unwrap();
        create_tables(&pool).await;

        Self { pool }
    }
}

// TODO: Move this shit out of here.
async fn wshandler(ws: WebSocketUpgrade, State(state): State<CarnyState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

// TODO: Move this shit out of here.
async fn websocket(stream: WebSocket, state: CarnyState) {
    let (mut sender, mut receiver) = stream.split();
    // If the incoming connection provides the matchserver's auth token, this is set.
    // let mut is_matchserver = false;
    let mut _token = String::new();
    let mut current_match = ResolvedOverwatchMatch::default();

    // Wait for message from matchserver
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(recv) = message {
            // If the server has just connected, it will immediately auth itself
            if recv.starts_with("auth:") {
                println!("{:#?}", recv);
                sender.send(Message::Text(String::from("auth ack"))).await;
                _token = recv.split(":").next().unwrap().to_string();
                // TODO: validate_matchserver(&token)
            }

            // The matchserver is asking if there's a pending match (status = 0)
            if recv == "match?" {
                // Check if there is
                if let Some(resolved_match) = overwatch_match::get_pending_match(&state.pool).await
                {
                    // Store the match data temporarily
                    current_match = resolved_match.clone();
                    // If there is, tell the matchserver to expect to receive the match now
                    if let Ok(_) = sender.send(Message::Text(String::from("match"))).await {
                        // Send the match
                        sender
                            .send(Message::Text(
                                serde_json::to_string(&resolved_match).unwrap(),
                            ))
                            .await;
                    }
                }
            }

            match recv.as_str() {
                "ack" => {
                    sender.send(Message::Text(String::from("ack"))).await;
                }
                // Matchserver is letting us know it has the match data for a match we've just sent it.
                "match ack" => {
                    // Need to update that match's row to show that it no longer needs to be sent to a matchserver
                    // We stored it in `current_match`.                                 1 = matchserver has the match.
                    match overwatch_match::set_match_status(
                        current_match.overwatch_match.id,
                        1,
                        &state.pool,
                    )
                    .await
                    {
                        Ok(_) => {}
                        Err(e) => eprintln!("{e}"),
                    }
                }
                "match lobby" => {
                    // sender.send(Message::Text(String::from("ack"))).await;
                    match overwatch_match::set_match_status(
                        current_match.overwatch_match.id,
                        2,
                        &state.pool,
                    )
                    .await
                    {
                        Ok(_) => {}
                        Err(e) => eprintln!("{e}"),
                    }
                }
                "match ingame" => {
                    // sender.send(Message::Text(String::from("ack"))).await;
                    match overwatch_match::set_match_status(
                        current_match.overwatch_match.id,
                        3,
                        &state.pool,
                    )
                    .await
                    {
                        Ok(_) => {}
                        Err(e) => eprintln!("{e}"),
                    }
                }
                // TODO: The matchserver needs to tell us who the winner is.
                "match completed" => {
                    // sender.send(Message::Text(String::from("ack"))).await;
                    match overwatch_match::set_match_status(
                        current_match.overwatch_match.id,
                        4,
                        &state.pool,
                    )
                    .await
                    {
                        Ok(_) => {}
                        Err(e) => eprintln!("{e}"),
                    }
                }
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        env::set_var("RUST_BACKTRACE", "1");
    }

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
        // TODO(aalhendi): build forgot pw page...
        .route("/settings/:settings_subroute", get(settings_route))
        .route("/queue", get(queue_route))
        .route("/play", get(queue_route))
        .route("/leaderboard", get(leaderboard_route))
        .route("/@:username", get(profile_route))
        // Components
        .route("/components/registration", get(register_form))
        .route("/components/login", get(login_form))
        .route("/components/hero", get(hero))
        .route("/components/leaderboard", get(leaderboard_comp))
        .route("/components/queue_table", get(queue_table))
        .route("/components/profile/:username", get(profile_comp))
        .route(
            "/components/settings/:settings_subroute",
            get(settings_user),
        )
        .route(
            "/components/queue_user_table/:username",
            get(queue_user_panel),
        )
        // Endpoints
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .route("/api/forgot-password", post(forgot_password))
        .route("/api/reset-password/", post(reset_password))
        .route("/api/join_queue", post(join_queue))
        .route("/api/leave_queue", post(leave_queue))
        .route("/api/settings_user", post(save_settings))
        // Websockets
        .route("/ws/notifications", get(wshandler))
        .layer(cors)
        .with_state(state.clone());

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
