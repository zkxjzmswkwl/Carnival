use easy_password::bcrypt::verify_password;
use axum::{
    extract::State,
    http::StatusCode,
    Json
};
use crate::{
    api::payloads::{RegisterInput, LoginInput},
    CarnyState,
    db::service
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

    if service::does_battletag_exist(battletag, &state.pool).await {
        return (StatusCode::BAD_REQUEST, "Battletag already exists".to_string())
    }

    if service::does_username_exist(username, &state.pool).await {
        return (StatusCode::BAD_REQUEST,
                "Username already exists".to_string());
    }

    match service::create_user(username, password, battletag, &state.pool).await {
        Ok(_) => return (StatusCode::OK, "Created".to_string()),
        Err(e) => {
            eprintln!("{e}");
            return (StatusCode::INTERNAL_SERVER_ERROR,
                    "Error creating user.".to_string());
        }
    }
}

pub async fn login(
    State(state): State<CarnyState>,
    Json(post_data): Json<LoginInput>
) -> (StatusCode, String) {

    let username: &str = post_data.get_username();
    let password: &str = post_data.get_password();

    if !service::does_username_exist(username, &state.pool).await {
        return (StatusCode::BAD_REQUEST, "User does not exist".to_string());
    }

    let user = service::user_by_username(username, &state.pool).await;
    if !user.is_ok() {
        return (StatusCode::INTERNAL_SERVER_ERROR,
            "Error fetching user".to_string());
    }

    if verify_password(password, user.unwrap().get_password(), b"dev-VERYsecure").unwrap() {
        return (StatusCode::OK, "totally-real-token".to_string());
    }

    return (StatusCode::BAD_REQUEST, "Incorrect username or password".to_string());
}
