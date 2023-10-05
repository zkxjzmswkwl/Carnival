use std::fs;

use crate::db::services::queue::pop_queue;
use crate::db::services::{bracket, password_reset_token, session_token as session};
use crate::{
    api::payloads::{LoginInput, RegisterInput},
    db::services::{queue, user},
    rendering::components::build_queue_comp,
    CarnyState, HMAC_KEY,
};
use axum::response::IntoResponse;
use axum::{
    body::{Bytes, Full},
    extract::State,
    response::Response,
    Json, TypedHeader,
};
use chrono::{Duration, Utc};
use easy_password::bcrypt::verify_password;
use headers::Cookie;
use http::{HeaderMap, HeaderValue, StatusCode};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use sqlx::SqlitePool;
use static_str_ops::static_format;
use uuid::Uuid;

use super::payloads::{
    ForgotPasswordInput, JoinQueueInput, LeaveQueueInput, ResetPasswordInput, UpdateSettingsInput,
};

async fn validate_session_cookie(user_id: i32, ip: &str, pool: &SqlitePool) -> bool {
    match session::validate(ip, user_id, pool).await {
        Some(_) => return true,
        None => return false,
    }
}

pub async fn register(
    State(state): State<CarnyState>,
    Json(post_data): Json<RegisterInput>,
) -> (StatusCode, String) {
    let username: &str = &post_data.username;
    let battletag: &str = &post_data.battletag;
    let email: &str = &post_data.email;
    let role: &str = &post_data.role;
    let password: &str = &post_data.password;
    let password_conf: &str = &post_data.password_conf;
    let bracket_key: &str = &post_data.bracket_key;

    if !vec!["Tank", "DPS", "Support"]
        .iter()
        .any(|x| role.contains(x))
    {
        return (StatusCode::BAD_REQUEST, "Role does not exist".to_string());
    }

    if password != password_conf {
        return (
            StatusCode::BAD_REQUEST,
            "Passwords do not match".to_string(),
        );
    }

    if user::does_battletag_exist(battletag, &state.pool).await {
        return (
            StatusCode::BAD_REQUEST,
            "Battletag already exists".to_string(),
        );
    }

    if user::does_username_exist(username, &state.pool).await {
        return (
            StatusCode::BAD_REQUEST,
            "Username already exists".to_string(),
        );
    }

    if user::does_email_exist(email, &state.pool).await {
        return (StatusCode::BAD_REQUEST, "Email already exists".to_string());
    }

    if !bracket::does_key_exist(bracket_key, &state.pool).await {
        return (
            StatusCode::BAD_REQUEST,
            "Bracket key does not exist.".to_string(),
        );
    }

    match user::create_user(username, password, battletag, email, role, &state.pool).await {
        Ok(query_result) => {
            // If the user provided a bracket key that wasn't default, add them to that bracket.
            if let Some(bracket) = bracket::by_key(bracket_key, &state.pool).await {
                bracket::add_user(query_result.last_insert_rowid(), bracket.id, &state.pool).await;
            }
            let redirect_js = fs::read_to_string("static/js/redirect_register.js")
                .unwrap_or("User created. Error redirecting.".to_string());
            (
                StatusCode::CREATED,
                format!("<script>{}</script>", redirect_js),
            )
        }
        Err(e) => {
            eprintln!("{e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error creating user.".to_string(),
            )
        }
    }
}

#[axum_macros::debug_handler]
pub async fn login(
    headers: HeaderMap,
    State(state): State<CarnyState>,
    Json(post_data): Json<LoginInput>,
) -> Response<Full<Bytes>> {
    let mut r: Response<Full<Bytes>> = Response::new(Full::from("nil"));

    let mut remote_addr = String::from("127.0.0.1");
    if let Some(addr_header_val) = headers.get("x-real-ip") {
        // cant be fucked right now.
        remote_addr = addr_header_val.to_str().unwrap().to_string();
    }

    let username: &str = &post_data.username;
    let password: &str = &post_data.password;
    let user_result = user::user_by_username(username, &state.pool).await;

    let user = match user_result {
        Ok(unwrapped_user) => unwrapped_user,
        // If there is no user by the username posted to us, error out.
        Err(_) => {
            *r.status_mut() = StatusCode::BAD_REQUEST;
            *r.body_mut() = Full::from("User does not exist");
            return r;
        }
    };

    if verify_password(password, &user.password, HMAC_KEY).unwrap() {
        // checks to see if the requesting user has a valid token already.
        let needs_token = session::token_by_user_id(user.id, &state.pool)
            .await
            .is_none();
        // If they don't, create one.
        if needs_token {
            let session_option = session::create(&remote_addr, user.id, &state.pool).await;
            // This would suck.
            if session_option.is_none() {
                *r.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                *r.body_mut() = Full::from("Could not create token");
                return r;
            }
        }

        // Validate that the token hasn't been stolen.
        // TODO: This shouldn't be happening here
        //      There's no reason to validate the session token on login.
        //      If someone knows a user's password, they know their password.
        //      They will still be assigned a token (as of now).
        //
        //      This *should* be tossed into middleware of some kind and then
        //      invoked prior to execution on any endpoint route that is denoted
        //      to require authorization.
        //
        match session::validate(&remote_addr, user.id, &state.pool).await {
            // If it hasn't, cool. Set the cookie and be done with it.
            Some(session) => {
                // Wait why the fuck is the message "User created" - This is the login?
                let redirect_js = fs::read_to_string("static/js/redirect_login.js")
                    // I did this?????? LOl
                    .unwrap_or("User created. Error redirecting.".to_string());

                *r.status_mut() = StatusCode::OK;
                *r.body_mut() = Full::from(format!("<script>{}</script>", redirect_js));

                let cookies_json = serde_json::to_string(&session).unwrap().to_string();
                r.headers_mut().insert(
                    "Set-Cookie",
                    HeaderValue::from_str(static_format!("session_id={};path=/;", cookies_json,))
                        .unwrap(),
                );
                // who u be men?
                // r.headers_mut().insert(
                //     "Set-Cookie",
                //     HeaderValue::from_str(static_format!("whoyoube={};path=/;", user.id)).unwrap(),
                // );
            }
            // If it has, we get very upset and tell the client that their actions are
            // "NOT_ACCEPTABLE".
            None => {
                *r.status_mut() = StatusCode::NOT_ACCEPTABLE;
                *r.body_mut() = Full::from("Fuck you?");
            }
        }
        return r;
    }

    // If we haven't dipped yet then this is the only remaining possibility.
    *r.status_mut() = StatusCode::BAD_REQUEST;
    *r.body_mut() = Full::from("Incorrect username or password");
    r
}

pub async fn forgot_password(
    State(state): State<CarnyState>,
    Json(post_data): Json<ForgotPasswordInput>,
) -> impl IntoResponse {
    // TODO(aalhendi): Validate email. Garde crate (https://github.com/jprochazk/garde) <- plug for the homie

    let maybe_user = user::user_by_email(&post_data.email, &state.pool).await;
    let user_id = match maybe_user {
        Ok(u) => u.id,
        Err(_) => todo!("Handle error silently, maybe return to user 'we sent a reset email if it exists in records'"),
    };

    let token = Uuid::new_v4().to_string();

    let expires_at = (Utc::now() + Duration::hours(2)).timestamp();
    // TODO(aalhendi): handle err maybe? HTTP 500
    password_reset_token::store_token(user_id, &token, expires_at, &state.pool).await;

    send_email(&post_data.email, &token).await;
    (
        StatusCode::OK,
        "Email will be sent if it exists in records :)",
    )
}

pub async fn reset_password(
    State(state): State<CarnyState>,
    Json(post_data): Json<ResetPasswordInput>,
) -> impl IntoResponse {
    let token = &post_data.token;
    let new_password = &post_data.new_password;

    match password_reset_token::validate_token(token, &state.pool).await {
        Ok(Some(user_id)) => {
            match password_reset_token::update_password(user_id, new_password, &state.pool).await {
                Ok(_) => {
                    password_reset_token::delete_token(token, &state.pool)
                        .await
                        .expect("Failed to delete token");
                    return (StatusCode::OK, "Password updated".to_string());
                }
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to update password".to_string(),
                    )
                }
            }
        }
        Ok(None) => {
            return (
                StatusCode::BAD_REQUEST,
                "Invalid or expired token".to_string(),
            )
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An error occurred".to_string(),
            )
        }
    }
}

// TODO(aalhendi): actually implement. this is literally the example on github modified...
// lettre (https://github.com/lettre/lettre)
async fn send_email(recipient_email: &str, token: &str) {
    let email = Message::builder()
        .from("no-reply@yourdomain.com".parse().unwrap())
        .from(recipient_email.parse().unwrap())
        .subject("Carnival Password Reset")
        .header(ContentType::TEXT_PLAIN)
        .body(format!("Here is your password reset token: {}", token))
        .unwrap();

    let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}

pub async fn join_queue(
    headers: HeaderMap,
    TypedHeader(cookies): TypedHeader<Cookie>,
    State(state): State<CarnyState>,
    Json(post_data): Json<JoinQueueInput>,
) -> (StatusCode, String) {
    let mut remote_addr = String::from("127.0.0.1");
    if let Some(addr_header_val) = headers.get("x-real-ip") {
        // cant be fucked right now.
        remote_addr = addr_header_val.to_str().unwrap().to_string();
    }

    let queue_id_i32: i32 = post_data.queue_id.parse().unwrap_or_default();
    if let Some(requesting_user) = user::from_cookies(&cookies, &state.pool).await {
        if !validate_session_cookie(requesting_user.id, &remote_addr, &state.pool).await {
            return (
                StatusCode::NOT_ACCEPTABLE,
                "Detected some funky stuff. Token invalidated.".to_string(),
            );
        }

        if queue::add_user_to_queue(
            queue_id_i32,
            requesting_user.id,
            &requesting_user.role,
            &state.pool,
        )
        .await
        .is_ok()
        {
            let new_match_option = pop_queue(queue_id_i32, &state.pool).await;
            if new_match_option.is_none() {
                println!("Queue did not pop");
                // Send websocket message to all users in the game
                // return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to pop queue.".to_string());
            }
            // ? - I don't want to go through shit and make everything (StatusCode, Option<String>)
            // but maybe it's worth it? idk. don't want to. 🐒
            return (
                StatusCode::OK,
                build_queue_comp(&cookies, &state.pool).await,
            );
        }
    }
    (StatusCode::OK, "Error joining queue.".to_string())
}

#[axum_macros::debug_handler]
pub async fn leave_queue(
    // ConnectInfo(connection): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    TypedHeader(cookies): TypedHeader<Cookie>,
    State(state): State<CarnyState>,
    Json(post_data): Json<LeaveQueueInput>,
) -> (StatusCode, String) {
    let mut remote_addr = String::from("127.0.0.1");
    if let Some(addr_header_val) = headers.get("x-real-ip") {
        // cant be fucked right now.
        remote_addr = addr_header_val.to_str().unwrap().to_string();
    }

    // TODO: This has no error handling.
    // All that can be used to determine if something's gone wrong
    // here, is the fact that `queue::delete_user_from_queue` calls `eprintln!`.
    // You're welcome 😎
    let queue_id_i32: i32 = post_data.queue_id.parse().unwrap_or_default();
    if let Some(requesting_user) = user::from_cookies(&cookies, &state.pool).await {
        if !validate_session_cookie(requesting_user.id, &remote_addr, &state.pool).await {
            return (
                StatusCode::NOT_ACCEPTABLE,
                "Detected some funky stuff. Token invalidated.".to_string(),
            );
        }

        queue::delete_user_from_queue(&queue_id_i32, &requesting_user.id, &state.pool).await;
        return (
            StatusCode::OK,
            build_queue_comp(&cookies, &state.pool).await,
        );
    }
    (StatusCode::OK, "Uhoh".to_string())
}

#[axum_macros::debug_handler]
pub async fn save_settings(
    State(state): State<CarnyState>,
    TypedHeader(cookies): TypedHeader<Cookie>,
    Json(post_data): Json<UpdateSettingsInput>,
) -> (StatusCode, String) {
    if let Some(requesting_user) = user::from_cookies(&cookies, &state.pool).await {
        println!("{:#?}", requesting_user);
        let battletag: &str = &post_data.battletag;
        let role: &str = &post_data.role;
        user::update_settings(requesting_user.id, battletag, role, &state.pool).await;
        return (StatusCode::OK, String::from("Updated"));
    }
    return (StatusCode::BAD_REQUEST, String::from("no"));
}
