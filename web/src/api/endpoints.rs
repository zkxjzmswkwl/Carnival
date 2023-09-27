use axum::{
    extract::State,
    Json
};
use easy_password::bcrypt::verify_password;
use crate::{
    api::payloads::{RegisterInput, LoginInput},
    CarnyState,
    db::service
};

pub async fn register(
    State(state): State<CarnyState>,
    Json(post_data): Json<RegisterInput>
)-> String {

    let username: &str = post_data.get_username();
    let password: &str = post_data.get_password();
    let password_conf: &str = post_data.get_password_conf();

    if password != password_conf {
        return "Passwords do not match".to_string();
    }

    if service::does_username_exist(username, &state.pool).await {
        return "Username already exists".to_string();
    }

    match service::create_user(username, password, &state.pool).await {
        Ok(_) => return "Ok".to_string(),
        Err(e) => {
            eprintln!("{e}");
            return "Error creating user.".to_string();
        }
    }
}

pub async fn login(
    State(state): State<CarnyState>,
    Json(post_data): Json<LoginInput>
) -> String {

    let username: &str = post_data.get_username();
    let password: &str = post_data.get_password();

    if !service::does_username_exist(username, &state.pool).await {
        return "User does not exist".to_string();
    }

    let user = service::user_by_username(username, &state.pool).await;
    if !user.is_ok() {
        return "Error fetching user".to_string();
    }

    if verify_password(password, user.unwrap().get_password(), b"dev-VERYsecure").unwrap() {
        return "totally-real-token".to_string();
    }
    return "Incorrect username or password".to_string();
}
