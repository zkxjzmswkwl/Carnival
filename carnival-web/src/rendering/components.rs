use axum::{extract::{Path, State}, TypedHeader};
use headers::Cookie;
use sqlx::SqlitePool;

use crate::{db::services::user, db::services::{queue::{ResolvedQueue, is_queued}, user::leaderboard_entries}, CarnyState, DOMAIN};

mod animations {
    pub fn table_domino(event_htmx_afterload: bool) -> String {
        // htmx:afterOnLoad
        let mut ret = String::from("<script>");
        if event_htmx_afterload {
            ret.push_str(r###"
                document.addEventListener("htmx:afterOnLoad", () => {
            "###);
        }

                
        ret.push_str(r###"
            const rows = document.querySelectorAll(".row");

            // Function to animate a single row
            function animateRow(row, delay) {
              anime({
                targets: row,
                opacity: [0, 1],
                translateY: [10, 0],
                easing: "easeOutExpo",
                duration: 800,
                delay: delay,
              });
            }

            // Loop through the rows and animate them with a delay
            rows.forEach((row, index) => {
              animateRow(row, index * 10); // Adjust the delay as needed
            });
        "###);

        if event_htmx_afterload {
            ret.push_str("});");
        }
        ret.push_str("</script>");
        ret
    }

    pub fn animated_header(header_text: &str) -> String {
        r###"
        <script>
          // cba. Let's us avoid Javascript's async execution from fucking us.
          document.getElementById('animated-header').textContent = document.getElementById('animated-header').textContent = "^.^"
          document.getElementById('animated-header').innerHTML = document.getElementById('animated-header').textContent.replace(/\S/g, '<span class=\"letter\">$&</span>');
          anime.timeline({loop: false})
            .add({
              targets: '#animated-header .letter',
              translateX: [40, 0],
              translateZ: 0,
              opacity: [0, 1],
              easing: 'easeOutExpo',
              duration: 700,
              delay: (el, i) => 500 + 30 * i
            }).add({
              targets: '#animated-header',
              backgroundSize: '100%',
              duration: 800,
              easing: 'easeOutExpo'
            });
          </script>
          "###.replace("^.^", header_text)
    }
}

mod utils {
  pub fn generate_table_row(values: &[&str]) -> String {
    let mut ret = "<tr>".to_string();
    for val in values {
      ret.push_str(&format!("<td class=\"row opacity-0\">{}</td>", val));
    }
    ret.push_str("</tr>");
    return ret;
  }
}

fn queue_button(is_queued: bool) -> String {
  match is_queued {
    true  => {
      r###"
        <input type="hidden" id="queue_id" name="queue_id" value="1" />
        <button 
          class="btn btn-md bg-[#1a8cd8] text-white md:w-25 lg:w-60 overflow-auto"
          hx-post="/api/leave_queue"
          hx-ext="json-enc"
          hx-include="[name='queue_id']"
          hx-target="#app">
        

          <span class="loading loading-infinity loading-md"></span>
          <div>Leave Queue</div>
        </button>
      "###.to_string()
    },
    false => {
      r###"
        <input type="hidden" name="queue_id" value="1" />
        <button
            class="btn btn-md bg-[#1a8cd8] text-white md:w-25 lg:w-60 overflow-auto"
            hx-post="/api/join_queue"
            hx-ext="json-enc"
            hx-include="[name='queue_id']"
            hx-target="#app">

          <!-- Not in queue -->
          <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" fill="currentColor" class="bi bi-play-fill" viewBox="0 0 16 16">
            <path d="m11.596 8.697-6.363 3.692c-.54.313-1.233-.066-1.233-.697V4.308c0-.63.692-1.01 1.233-.696l6.363 3.692a.802.802 0 0 1 0 1.393z"/>
          </svg>
          <div>Join Queue</div>
        </button>
      "###.to_string()
    }
  }
}



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
    format!(r###"<div class="container mt-4 mx-auto w-1/4 bg-base-200 p-6 rounded-lg">
      <div class="mb-3"><span id="animated-header" class="text-2xl text-white font-bold"></span></div>
      <form hx-post="{}/api/register" hx-ext="json-enc" class="join join-vertical w-full">
        <input name="username" type="text" placeholder="Username" class="input input-bordered rounded-lg mb-2 w-full">
        <input name="battletag" type="text" placeholder="Battletag (Case sensitive)" class="input input-bordered rounded-lg mb-2 w-full">
        <select name="role" class="select select-bordered w-full mb-2">
          <option disabled selected>Role</option>
          <option>Tank</option>
          <option>DPS</option>
          <option>Support</option>
        </select>
        <input name="password" type="password" placeholder="Password" class="input input-bordered rounded-lg mb-2 w-full">
        <input name="password_conf" type="password" placeholder="Password confirmation" class="input input-bordered rounded-lg mb-2 w-full">
        <button class="btn btn-wide bg-[#1a8cd8] text-white w-full">Register</button>
      </form>
    </div>
    {}"###, DOMAIN, animations::animated_header("Register"))
}

pub async fn login_form() -> String {
    format!(r###"
    <div class="container mt-4 mx-auto w-1/4 bg-base-200 p-6 rounded-lg">
      <div class="mb-3"><span id="animated-header" class="text-2xl text-white font-bold"></span></div>
      <form hx-post="{}/api/login" hx-ext="json-enc" class="join join-vertical w-full">
        <input name="username" type="text" placeholder="Username" class="input input-bordered rounded-lg mb-2 w-full">
        <input name="password" type="password" placeholder="Password" class="input input-bordered rounded-lg mb-2 w-full">
        <button class="btn btn-wide bg-[#1a8cd8] text-white w-full">Login</button>
      </form>
    </div>
    {},
    "###, DOMAIN, animations::animated_header("Login"))
}

#[axum_macros::debug_handler]
pub async fn leaderboard_comp(
  State(state): State<CarnyState>,
) -> String {

    let mut rows = String::new();
    let leaderboard_result = leaderboard_entries(&state.pool).await;
    if leaderboard_result.is_ok() {
      for entry in leaderboard_result.unwrap() {
        rows.push_str(
            &utils::generate_table_row(&[
                &entry.username, 
                &format!("{}", entry.rating), 
                &format!("{}", entry.wins), 
                &format!("{}", entry.losses), 
                &entry.role
            ])
        )
      }
    }
    format!(r###"
    {}
      <div class="cotainer p-4 bg-base-200 ovrflow-x-auto mx-auto w-1/2 mt-4">
          <div clas="flex flex-col mb-2">
            <div class="mb-3"><span id="animated-header" class="text-2xl text-white font-bold"></span></div>
          </div>

          <table class="table">
              <thead class="bg-base-300">
                  <tr class="boder-bottom border-[#1a8cd8]">
                      <th class="text-lg">Player</th>
                      <th class="text-lg">Rating</th>
                      <th class="text-lg">Wins</th>
                      <th class="text-lg">Losses</th>
                      <th class="text-lg">Role</th>
                  </tr>
                  </thead>
                  <tbody>
                  {}
                  </tbody>
              </thead>
          </table>
      </div>
    {}"###, animations::table_domino(false), rows, animations::animated_header("Leaderboard"))

}

pub async fn base(
    pool: &SqlitePool,
    cookies: &Cookie,
) -> String {

    let authed_items = ["Leaderboard", "Play", "Settings"];
    let noauth_items = ["Leaderboard", "Register", "Login"];

    let user_option = user::from_cookies(&cookies, pool).await;
    let user = user_option.unwrap_or_default();
    println!("{:#?}", user);

    let mut header_list: String = String::new();

    if user.id == 0 {
        for item in noauth_items {
            header_list.push_str(
                &format!("<li><a href=\"{}\">{}</a></li>", item.to_lowercase(), item)
            )
        }
    } else {
        for item in authed_items {
            header_list.push_str(
                &format!("<li><a href=\"{}\">{}</a></li>", item.to_lowercase(), item)
            )
        }
    }

    format!(r###"
        <html>
          <head>
            <title>Carnival</title>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <!-- HTMX -->
            <script src="https://unpkg.com/htmx.org@1.9.6"></script>
            <!-- HTMX json-enc extension -->
            <script src="https://unpkg.com/htmx.org/dist/ext/json-enc.js"></script>
            <!-- DaisyUI (Tailwind components etc) -->
            <link href="https://cdn.jsdelivr.net/npm/daisyui@3.8.1/dist/full.css" rel="stylesheet" type="text/css" />
            <!-- Tailwind -->
            <script src="https://cdn.tailwindcss.com"></script>
            <!-- Poppins font import -->
            <link rel="preconnect" href="https://fonts.googleapis.com">
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
            <link href="https://fonts.googleapis.com/css2?family=Poppins:wght@300;400;500;700&display=swap" rel="stylesheet">
            <!-- Devicons -->
            <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/devicons/devicon@v2.15.1/devicon.min.css">
            <!-- Anime.js (not weeb shit, animations) -->
            <script src="https://cdnjs.cloudflare.com/ajax/libs/animejs/2.0.2/anime.min.js"></script>

            <!-- I'm not sure it even matters if we shove this into its own file or not. Like, sure, it's not being minified. But do we fucking care? Honestly? -->
            <style>
              body {{
                font-family: "Poppins"
              }}
              .svg-in-button {{
                fill: #fff !important;
              }}
              #animated-header {{
                background-image: linear-gradient(transparent calc(97% - 1px), #1a8cd8 2px);
                background-size: 0;
                background-repeat: no-repeat;
                display: infinite;
              }}
              #animated-header .letter {{
                display: inline-block;
              }}
            </style>
          </head>
          <body>

            <!-- HEADER START -->
            <div class="navbar bg-base-300">
              <!-- Lefthand side -->
              <div class="navbar-start">
                <a href="/" class="btn btn-ghost normal-case text-xl">Carnival</a>
              </div>

              <!-- Center -->
              <div class="navbar-center">
                <ul class="menu menu-horizontal px-1">
                {}
                </ul>
              </div>

              <!-- Righthand side -->
              <div class="navbar-end">
                <img class="w-10 rounded-full" src="https://i.imgur.com/RfOQHPc.png" />
              </div>
            </div>
            <!-- HEADER END   -->
            <div id="app">
                <div class="flex h-screen">
                    <span class="m-auto loading loading-bars loading-lg"></span>
                </div>
            </div>
            https://www.youtube.com/watch?v=dQw4w9WgXcQ
          </body>
        </html>
    "###, header_list)
}

pub async fn build_queue_comp(
  cookies: &Cookie,
  pool: &SqlitePool
) -> String {

  // Only care about one queue for now
  let resolved_user = user::from_cookies(&cookies, pool).await;
  if resolved_user.is_none() {
    return "Couldn't be authenticated".to_string();
  }

  let resolved_queue = ResolvedQueue::from_id(1, pool).await;
  let mut tank_rows = String::new();
  let mut dps_rows = String::new();
  let mut support_rows = String::new();

  for tank in resolved_queue.tanks.iter() {
    tank_rows.push_str(&utils::generate_table_row(&[&tank.username, &tank.role]));
  }
  for dps in resolved_queue.dps.iter() {
    dps_rows.push_str(&utils::generate_table_row(&[&dps.username, &dps.role]));
  }
  for support in resolved_queue.supports.iter() {
    support_rows.push_str(&utils::generate_table_row(&[&support.username, &support.role]));
  }

  format!(
    r###"{}{}
      <div class="cotainer p-4 bg-base-200 ovrflow-x-auto mx-auto w-1/2 mt-4">
          <div clas="flex flex-col mb-2">
              <!-- Queue title, changes for each queue -->
              <div class="mb-3"><span id="animated-header" class="text-3xl text-white font-bold"></span></div>
              
              <!-- User information (Username, avatar, win/loss, rating, %, etc.) -->
              <div id="queue-user-panel">Loading</div>
              <div hx-get="{}/components/queue_user_table/{}" hx-trigger="load" hx-target="#queue-user-panel""></div>
          </div>

          <table class="table">
              <thead class="bg-base-300">
                  <tr class="boder-bottom border-[#1a8cd8]">
                      <th class="text-lg">Player</th>
                      <th class="text-lg">Role</th>
                  </tr>
                  </thead>
                  <tbody>
                          <!-- Tanks -->
                          {}
                          <!-- Dps -->
                          {}
                          <!-- Supports -->
                          {}
                  </tbody>
              </thead>
          </table>
      </div>
  </div>
  "###,
    animations::table_domino(true),
    animations::animated_header("Queue"),
    DOMAIN,
    resolved_user.unwrap().username,
    tank_rows,
    dps_rows,
    support_rows
  )
}

/// Serves purely static data atm. Will finish when I wake up - Carter
pub async fn queue_table(
  State(state): State<CarnyState>,
  TypedHeader(cookies): TypedHeader<Cookie>
) -> String {
    build_queue_comp(&cookies, &state.pool).await
}

pub async fn queue_user_panel(
  Path(username): Path<String>,
  State(state): State<CarnyState>
) -> String {

  format!(r###"
  <div class="flex flex-row justify-between mb-2">
      <div class="text-lg text-[#ddd] mb-2 pl-1 pt-[8px]">{}</div>
      {}
  </div>
  "###,
  &username,
  queue_button(is_queued(1, &username, &state.pool).await)) 
}
