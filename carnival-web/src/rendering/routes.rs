use axum::response::Html;
use crate::DOMAIN;
use super::components::base;

pub async fn index() -> Html<String> {
    Html(base().await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            &format!(r###"<div hx-get="{}/components/hero" hx-trigger="load" hx-target="#app""></div>"###, DOMAIN),
        )
    )
}

pub async fn login_route() -> Html<String> {
    Html(base().await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            &format!(r###"<div hx-get="{}/components/login" hx-trigger="load" hx-target="#app""></div>"###, DOMAIN),
        )
    )
}

pub async fn register_route() -> Html<String> {
    Html(base().await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            &format!(r###"<div hx-get="{}/components/registration" hx-trigger="load" hx-target="#app""></div>"###, DOMAIN),
        )
    )
}

pub async fn leaderboard_route() -> Html<String> {
    Html(base().await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            &format!(r###"<div hx-get="{}/components/leaderboard" hx-trigger="load" hx-target="#app""></div>"###, DOMAIN)
        )
    )
}

#[axum_macros::debug_handler]
pub async fn queue_route() -> Html<String> {
    Html(base().await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            &format!(r###"<div hx-get="{}/components/queue_table" hx-trigger="load" hx-target="#app""></div>"###, DOMAIN),
        )
    )
}

/// NOT IMPLEMENTED
pub async fn profile_route() -> Html<String> {
    Html(base().await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            &format!(r###"<div hx-get="{}/components/profile" hx-trigger="load" hx-target="#app""></div>"###, DOMAIN),
        )
    )
}

