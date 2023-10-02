use axum::{response::Html, extract::State, TypedHeader};
use headers::Cookie;
use crate::{DOMAIN, CarnyState};
use super::components::base;

pub async fn index(
    State(state): State<CarnyState>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> Html<String> {
    Html(base(&state.pool, &cookies).await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            &format!(r###"<div hx-get="{}/components/hero" hx-trigger="load" hx-target="#app""></div>"###, DOMAIN),
        )
    )
}

pub async fn login_route(
    State(state): State<CarnyState>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> Html<String> {
    Html(base(&state.pool, &cookies).await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            &format!(r###"<div hx-get="{}/components/login" hx-trigger="load" hx-target="#app""></div>"###, DOMAIN),
        )
    )
}

pub async fn register_route(
    State(state): State<CarnyState>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> Html<String> {
    Html(base(&state.pool, &cookies).await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            &format!(r###"<div hx-get="{}/components/registration" hx-trigger="load" hx-target="#app""></div>"###, DOMAIN),
        )
    )
}

pub async fn leaderboard_route(
    State(state): State<CarnyState>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> Html<String> {
    Html(base(&state.pool, &cookies).await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            &format!(r###"<div hx-get="{}/components/leaderboard" hx-trigger="load" hx-target="#app""></div>"###, DOMAIN)
        )
    )
}

pub async fn queue_route(
    State(state): State<CarnyState>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> Html<String> {
    Html(base(&state.pool, &cookies).await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            &format!(r###"<div hx-get="{}/components/queue_table" hx-trigger="load" hx-target="#app""></div>"###, DOMAIN),
        )
    )
}

/// NOT IMPLEMENTED
pub async fn profile_route(
    State(state): State<CarnyState>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> Html<String> {
    Html(base(&state.pool, &cookies).await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            &format!(r###"<div hx-get="{}/components/profile" hx-trigger="load" hx-target="#app""></div>"###, DOMAIN),
        )
    )
}

