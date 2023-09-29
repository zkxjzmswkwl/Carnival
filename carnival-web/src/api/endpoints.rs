use std::net::SocketAddr;

use easy_password::bcrypt::verify_password;
use axum::{
    extract::{State, ConnectInfo},
    http::StatusCode,
    Json
};
use crate::{
    api::payloads::{RegisterInput, LoginInput},
    CarnyState,
    db::services::{user::{userid_by_username, self}, session_token::create_session_token}, HMAC_KEY
};

pub async fn register(
    State(state): State<CarnyState>,
    Json(post_data): Json<RegisterInput>
)-> (StatusCode, String) {

    let username: &str = post_data.get_username();
    let password: &str = post_data.get_password();
    let password_conf: &str = post_data.get_password_conf();
    let battletag: &str = post_data.get_battletag();

    if password != password_conf {
        return (StatusCode::BAD_REQUEST,
                "Passwords do not match".to_string());
    }

    if user::does_battletag_exist(battletag, &state.pool).await {
        return (StatusCode::BAD_REQUEST, "Battletag already exists".to_string())
    }

    if user::does_username_exist(username, &state.pool).await {
        return (StatusCode::BAD_REQUEST,
                "Username already exists".to_string());
    }

    match user::create_user(username, password, battletag, &state.pool).await {
        Ok(_) => return (StatusCode::OK, "Created".to_string()),
        Err(e) => {
            eprintln!("{e}");
            return (StatusCode::INTERNAL_SERVER_ERROR,
                    "Error creating user.".to_string());
        }
    }
}

#[axum_macros::debug_handler]
pub async fn login(
    ConnectInfo(conn): ConnectInfo<SocketAddr>,
    State(state): State<CarnyState>,
    Json(post_data): Json<LoginInput>
) -> (StatusCode, String) {

    println!("{:#?}", conn);
    let mut remote_addr = conn.to_string();
    if remote_addr.contains(":") {
        remote_addr = remote_addr.split(":").next().unwrap().to_string();
        println!("{remote_addr}")
    }
    let username: &str = post_data.get_username();
    let password: &str = post_data.get_password();

    // TODO: This can and should be consolidated into one query, see above TODO.
    if !user::does_username_exist(username, &state.pool).await {
        return (StatusCode::BAD_REQUEST, "User does not exist".to_string());
    }

    let user = user::user_by_username(username, &state.pool).await;
    if !user.is_ok() {
        return (StatusCode::INTERNAL_SERVER_ERROR,
            "Error fetching user".to_string());
    }

    if verify_password(password, user.unwrap().get_password(), HMAC_KEY).unwrap() {
        // TODO: This can and should be consolidated into one query, see above TODO.
        let userid = userid_by_username(username, &state.pool).await;
        if userid.is_none() {
            return (StatusCode::BAD_REQUEST, "User does not exist".to_string());
        }

        let session_token = create_session_token(&remote_addr, userid.unwrap(), &state.pool).await ;
        if session_token.is_none() {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Could not create token".to_string());
        }
        return (StatusCode::OK, session_token.unwrap());
    }

    return (StatusCode::BAD_REQUEST, "Incorrect username or password".to_string());
}
