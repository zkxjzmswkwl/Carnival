use axum::{
    extract::{Path, State},
    TypedHeader,
};
use headers::Cookie;
use sqlx::SqlitePool;

use crate::{
    db::services::user,
    db::services::{
        queue::{is_queued, ResolvedQueue},
        user::leaderboard_entries,
    },
    CarnyState, DOMAIN,
};

mod animations {
    use std::fs;

    use super::utils;

    // I super give a fuck about this
    pub fn table_domino(event_htmx_afterload: bool) -> String {
        let mut ret = String::from("<script>");
        let event_listener_head: &'static str =
            "document.addEventListener(\"htmx:afterOnLoad\", () => {";

        let file_contents = fs::read_to_string("static/js/table_domino.js")
            .unwrap_or("<script>console.log('ruhroh disk boom')".to_string());

        if event_htmx_afterload {
            ret.push_str(event_listener_head);
        }
        ret.push_str(&file_contents);
        if event_htmx_afterload {
            ret.push_str("});");
        }
        ret.push_str("</script>");
        ret
    }

    pub fn animated_header(header_text: &str) -> String {
        utils::load_file_replace_shit("static/js/header_domino.js", &[("^.^", header_text)])
    }
}

mod utils {
    use std::fs;

    pub fn generate_table_row(values: &[&str]) -> String {
        let mut ret = "<tr>".to_string();
        for val in values {
            ret.push_str(&format!(
                "<td class=\"row opacity-0\"><a href=\"/@{}\">{}</a></td>",
                val, val
            ));
        }
        ret.push_str("</tr>");
        return ret;
    }

    pub fn read_to_fucking_string(file_path: &str) -> String {
        fs::read_to_string(file_path)
            .unwrap_or("".to_string())
            .to_string()
    }

    pub fn load_file_replace_shit(file_path: &str, shit: &[(&str, &str)]) -> String {
        let mut file_contents = read_to_fucking_string(file_path);
        // Every time this iterates it deallocates `file_contents` then reallocates that bitch
        // Don't replace a ton of shit
        for x in shit {
            file_contents = file_contents.replace(x.0, x.1);
        }
        file_contents
    }
}

fn queue_button(is_queued: bool) -> String {
    match is_queued {
        true => utils::read_to_fucking_string("static/html/queue_button_queued.html"),
        false => utils::read_to_fucking_string("static/html/queue_button_notqueued.html"),
    }
}

// Leave this as &'static str - it's not dynamic so we don't give a shit.
pub async fn hero() -> &'static str {
    r###"
    <div class="hero min-h-screen" style="background-image: url(https://i.imgur.com/WT1Un8q.jpg);">
        <div class="hero-overlay bg-opacity-50"></div>
        <div class="hero-content text-center text-white">
            <div class="max-w-md">
                <h1 class="mb-5 text-5xl font-bold">Carnival</h1>
                <p class="mb-5">Open source instanced ladder.</p>
                <a href="https://github.com/zkxjzmswkwl/Carnival" target="_blank" class="btn bg-[#1a8cd8] text-white">
                    <i class="text-2xl devicon-github-original"></i>
                    Source code
                </a>
            </div>
        </div>
    </div>
    "###
}

pub async fn register_form() -> String {
    utils::load_file_replace_shit(
        "static/html/register.html",
        &[
            ("domain_", DOMAIN),
            (">//<", &animations::animated_header("Register")),
        ],
    )
}

pub async fn login_form() -> String {
    utils::load_file_replace_shit(
        "static/html/login.html",
        &[
            ("domain_", DOMAIN),
            (">//<", &animations::animated_header("Login")),
        ],
    )
}

pub async fn leaderboard_comp(State(state): State<CarnyState>) -> String {
    let mut rows = String::new();
    let leaderboard_result = leaderboard_entries(&state.pool).await;
    if leaderboard_result.is_ok() {
        for entry in leaderboard_result.unwrap() {
            rows.push_str(&utils::generate_table_row(&[
                &entry.username,
                &entry.battletag,
                &format!("{}", entry.rating),
                &format!("{}", entry.wins),
                &format!("{}", entry.losses),
                &entry.role,
            ]))
        }
    }

    utils::load_file_replace_shit(
        "static/html/leaderboard.html",
        &[
            ("table_domino_js_", &animations::table_domino(false)),
            ("rows_", &rows),
            (
                "animated_header_",
                &animations::animated_header("Leaderboard"),
            ),
        ],
    )
}

pub async fn base(pool: &SqlitePool, cookies: &Cookie) -> String {
    let authed_items = [
        ("Leaderboard", "leaderboard"),
        ("Play", "play"),
        ("Settings", "settings/user"),
    ];
    let noauth_items = [
        ("Leaderboard", "leaderboard"),
        ("Register", "register"),
        ("Login", "login"),
    ];

    let user_option = user::from_cookies(&cookies, pool).await;
    let user = user_option.unwrap_or_default();

    let mut header_list: String = String::new();

    if user.id == 0 {
        for item in noauth_items {
            header_list.push_str(&format!("<li><a href=\"/{}\">{}</a></li>", item.1, item.0))
        }
    } else {
        for item in authed_items {
            header_list.push_str(&format!("<li><a href=\"/{}\">{}</a></li>", item.1, item.0))
        }
    }

    utils::load_file_replace_shit("static/html/base.html", &[("header_items_", &header_list)])
}

pub async fn build_queue_comp(cookies: &Cookie, pool: &SqlitePool) -> String {
    // Only care about one queue for now
    let resolved_user = user::from_cookies(&cookies, pool).await;
    if resolved_user.is_none() {
        return "<div class=\"text-xl text-center\">Couldn't be authenticated</div>".to_string();
    }

    let resolved_queue = ResolvedQueue::from_id(1, pool).await;
    let mut tank_rows = String::new();
    let mut dps_rows = String::new();
    let mut support_rows = String::new();

    for tank in resolved_queue.tanks.iter() {
        tank_rows.push_str(&utils::generate_table_row(&[
            &tank.battletag,
            &tank.role,
            &tank.rating.to_string(),
            &tank.wins.to_string(),
            &tank.losses.to_string(),
        ]));
    }
    for dps in resolved_queue.dps.iter() {
        dps_rows.push_str(&utils::generate_table_row(&[
            &dps.battletag,
            &dps.role,
            &dps.rating.to_string(),
            &dps.wins.to_string(),
            &dps.losses.to_string(),
        ]));
    }
    for support in resolved_queue.supports.iter() {
        support_rows.push_str(&utils::generate_table_row(&[
            &support.battletag,
            &support.role,
            &support.rating.to_string(),
            &support.wins.to_string(),
            &support.losses.to_string(),
        ]));
    }

    utils::load_file_replace_shit(
        "static/html/queue.html",
        &[
            ("table_domino_js_", &animations::table_domino(true)),
            ("animated_header_", &animations::animated_header("Queue")),
            ("domain_", DOMAIN),
            ("_username_", &resolved_user.unwrap().username),
            ("tank_rows_", &tank_rows),
            ("dps_rows_", &dps_rows),
            ("support_rows_", &support_rows),
        ],
    )
}

pub async fn queue_table(
    State(state): State<CarnyState>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> String {
    build_queue_comp(&cookies, &state.pool).await
}

pub async fn queue_user_panel(
    Path(username): Path<String>,
    State(state): State<CarnyState>,
) -> String {
    utils::load_file_replace_shit(
        "static/html/queue_user_panel.html",
        &[
            ("_username_", &username),
            // lol
            (
                "queue_button_",
                &queue_button(is_queued(1, &username, &state.pool).await),
            ),
        ],
    )
}

pub async fn profile_comp(Path(username): Path<String>, State(state): State<CarnyState>) -> String {
    let user = user::user_by_username(&username, &state.pool)
        .await
        .unwrap_or_default();

    utils::load_file_replace_shit(
        "static/html/profile.html",
        &[
            (
                "animated_header_",
                &animations::animated_header(&format!("{} - {}", &username, &user.role)),
            ),
            ("_rating_", &user.rating.to_string()),
            ("_wins_", &user.wins.to_string()),
            ("_losses_", &user.losses.to_string()),
            // lol
            (
                "queue_button_",
                &queue_button(is_queued(1, &username, &state.pool).await),
            ),
        ],
    )
}

pub async fn settings_user() -> String {
    utils::load_file_replace_shit(
        "static/html/settings_user.html",
        &[
            ("domain_", DOMAIN),
            (">//<", &animations::animated_header("User Settings")),
        ],
    )
}
