use axum::response::Html;

pub async fn register_form() -> &'static str {
    r###"<div class="container mt-4 mx-auto w-1/4 bg-base-200 p-12 rounded-lg">
      <form hx-post="http://localhost:3000/api/register" hx-ext="json-enc" class="join join-vertical w-full">
        <input name="username" type="text" placeholder="Username" class="input input-bordered rounded-lg mb-2 w-full">
        <input name="battletag" type="text" placeholder="Battletag (Case sensitive)" class="input input-bordered rounded-lg mb-2 w-full">
        <input name="password" type="password" placeholder="Password" class="input input-bordered rounded-lg mb-2 w-full">
        <input name="password_conf" type="password" placeholder="Password confirmation" class="input input-bordered rounded-lg mb-2 w-full">
        <button class="btn btn-wide bg-[#1a8cd8] text-white w-full">Register</button>
      </form>
    </div>"###
}

pub async fn login_form() -> &'static str {
    r###"
    <div class="container mt-4 mx-auto w-1/4 bg-base-200 p-12 rounded-lg">
      <form hx-post="http://localhost:3000/api/login" hx-ext="json-enc" class="join join-vertical w-full">
        <input name="username" type="text" placeholder="Username" class="input input-bordered rounded-lg mb-2 w-full">
        <input name="password" type="password" placeholder="Password" class="input input-bordered rounded-lg mb-2 w-full">
        <button class="btn btn-wide bg-[#1a8cd8] text-white w-full">Login</button>
      </form>
    </div>"###
}

pub async fn index() -> String {
    r###"
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
            <!-- Our shit -->
            <link rel="stylesheet" href="main.css">
            <!-- Poppins font import -->
            <link rel="preconnect" href="https://fonts.googleapis.com">
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
            <link href="https://fonts.googleapis.com/css2?family=Poppins:wght@300;400;500;700&display=swap" rel="stylesheet">
          </head>
          <body>

            <!-- HEADER START -->
            <div class="navbar bg-base-200">
              <!-- Lefthand side -->
              <div class="navbar-start">
                <a class="btn btn-ghost normal-case text-xl">Carnival</a>
              </div>

              <!-- Center -->
              <div class="navbar-center">
                <ul class="menu menu-horizontal px-1">
                  <li><a>Leaderboard</a></li>
                  <!-- TODO: Implement an isAuthed check, display Play, Settings if authed. If not, display Register, Login -->
                  <li><a>Play</a></li>
                  <li><a href="/register">Register</a></li>
                  <li><a href="/login">Login</a></li>
                  <li><a>Settings</a></li>
                </ul>
              </div>

              <!-- Righthand side -->
              <div class="navbar-end">
                <img class="w-10 rounded-full" src="https://pbs.twimg.com/profile_images/1605042959858036736/l8bFPAzw_400x400.jpg" />
              </div>
            </div>
            <!-- HEADER END   -->
            <div id="app">Loading</div>
            https://www.youtube.com/watch?v=dQw4w9WgXcQ
          </body>
        </html>
    "###.to_string()
}

pub async fn login_route() -> Html<String> {
    Html(index().await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            r###"<div hx-get="http://localhost:3000/components/login" hx-trigger="load" hx-target="#app""></div>"###
        )
    )
}

pub async fn register_route() -> Html<String> {
    Html(index().await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            r###"<div hx-get="http://localhost:3000/components/registration" hx-trigger="load" hx-target="#app""></div>"###
        )
    )
}
