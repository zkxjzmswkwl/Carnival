use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub role: String,
    pub password: String,
    pub password_conf: String,
    pub battletag: String,
    pub email: String,
    pub bracket_key: String
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct ForgotPasswordInput {
    pub email: String,
}


#[derive(Deserialize)]
pub struct ResetPasswordInput {
    pub token: String,
    pub new_password: String,
}

// See LeaveQueueInput comment.
#[derive(Deserialize, Debug)]
pub struct JoinQueueInput {
    pub queue_id: String,
}

// Feels wrong so it probably is.
// The leave_queue endpoint infers the requesting user
// from request header cookies. So we really only need the queue_id.
// But I'm not sure how to set Axum's `Json<Type>` to an inlined struct or something?
// No idea.
#[derive(Deserialize, Debug)]
pub struct LeaveQueueInput {
    pub queue_id: String,
}

#[derive(Deserialize)]
pub struct UpdateSettingsInput {
    pub battletag: String,
    pub role: String,
}