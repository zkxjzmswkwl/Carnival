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
    db::services::user::{self}, HMAC_KEY
};
use crate::db::services::session_token as session;

pub async fn register(
    State(state): State<CarnyState>,
    Json(post_data): Json<RegisterInput>
)-> (StatusCode, String) {

    // NOTE(aalhendi): is this needed?
    let username: &str = &post_data.username;
    let password: &str = &post_data.password;
    let password_conf: &str = &post_data.password_conf;
    let battletag: &str = &post_data.battletag;

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
        Ok(_) => (StatusCode::OK, "Created".to_string()),
        Err(e) => {
            eprintln!("{e}");
            (StatusCode::INTERNAL_SERVER_ERROR,
                    "Error creating user.".to_string())
        }
    }
}

#[axum_macros::debug_handler]
pub async fn login(
    ConnectInfo(connection): ConnectInfo<SocketAddr>,
    State(state): State<CarnyState>,
    Json(post_data): Json<LoginInput>
) -> (StatusCode, String) {

    let username: &str = &post_data.username;
    let password: &str = &post_data.password;

    let user_result = user::user_by_username(username, &state.pool).await;
    let user = match user_result {
        Ok(unwrapped_user) => unwrapped_user,
        Err(_) => return (StatusCode::BAD_REQUEST, "User does not exist".to_string())
    };

    if verify_password(password, &user.password, HMAC_KEY).unwrap() {
        let needs_token = session::token_by_user_id(user.id, &state.pool).await.is_none();
        if needs_token {
            let session = session::create(&connection, user.id, &state.pool).await;
            if session.is_none() {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Could not create token".to_string());
            }
            return (StatusCode::OK, session.unwrap());
        }

        // This would typically mean someone's being a lil illicit (stealing someone's token)
        //      HELP-WANTED: Automatically file fbi.gov ic3 report. Cyber crime is no joke.
        //      Very serious. So super serious.
        match session::validate(&connection, user.id, &state.pool).await {
            Some(token) => return (StatusCode::OK, token),
            None        => return (StatusCode::NOT_ACCEPTABLE, "Fuck you?".to_string())
        }
    }

    (StatusCode::BAD_REQUEST, "Incorrect username or password".to_string())
}
