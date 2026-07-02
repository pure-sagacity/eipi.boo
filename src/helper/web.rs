use std::sync::Arc;

use axum::Router;
use axum::extract::{Path, State};
use axum::http::header;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use log::info;

use crate::server::AppState;

fn landing_page(
    confessions: i64,
    humans: i64,
    replies: i64,
    reactions: i64,
    online: usize,
) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>eipi.boo</title>
  <meta property="og:title" content="eipi.boo">
  <meta property="og:description" content="{} confessions from {} strangers over ssh">
  <meta property="og:type" content="website">
  <style>
    * {{ margin: 0; padding: 0; box-sizing: border-box; }}
    body {{
      background: #faf4ed;
      color: #575279;
      font-family: 'Courier New', monospace;
      display: flex;
      justify-content: center;
      align-items: center;
      min-height: 100vh;
    }}
    .container {{
      text-align: center;
      padding: 2rem;
    }}
    h1 {{
      font-size: 3rem;
      color: #b4637a;
      margin-bottom: 0.5rem;
    }}
    .tagline {{
      color: #9893a5;
      font-size: 1.1rem;
      margin-bottom: 2rem;
    }}
    .stats {{
      display: flex;
      justify-content: center;
      gap: 2rem;
      margin-bottom: 2rem;
      flex-wrap: wrap;
    }}
    .stat {{
      text-align: center;
    }}
    .stat-num {{
      font-size: 1.8rem;
      color: #b4637a;
      font-weight: bold;
      display: block;
    }}
    .stat-label {{
      font-size: 0.8rem;
      color: #9893a5;
    }}
    .cmd {{
      background: #f2e9e1;
      border: 1px solid #dfdad9;
      border-radius: 8px;
      padding: 1.2rem 2rem;
      display: inline-block;
      margin-bottom: 1.5rem;
    }}
    .cmd span {{
      color: #56949f;
      font-size: 1.3rem;
    }}
    .cmd code {{
      color: #286983;
      font-size: 1.3rem;
      font-weight: bold;
    }}
    .online {{
      color: #56949f;
      font-size: 0.85rem;
      margin-bottom: 1.5rem;
    }}
    .online .dot {{
      display: inline-block;
      width: 8px;
      height: 8px;
      background: #56949f;
      border-radius: 50%;
      margin-right: 4px;
      animation: pulse 2s infinite;
    }}
    @keyframes pulse {{
      0%, 100% {{ opacity: 1; }}
      50% {{ opacity: 0.4; }}
    }}
    .footer {{
      color: #9893a5;
      font-size: 0.85rem;
    }}
    .footer a {{
      color: #907aa9;
      text-decoration: none;
    }}
    .footer a:hover {{
      text-decoration: underline;
    }}
  </style>
</head>
<body>
  <div class="container">
    <h1>eipi.boo</h1>
    <p class="tagline">confess over ssh</p>
    <div class="stats">
      <div class="stat">
        <span class="stat-num">{}</span>
        <span class="stat-label">confessions</span>
      </div>
      <div class="stat">
        <span class="stat-num">{}</span>
        <span class="stat-label">humans</span>
      </div>
      <div class="stat">
        <span class="stat-num">{}</span>
        <span class="stat-label">replies</span>
      </div>
      <div class="stat">
        <span class="stat-num">{}</span>
        <span class="stat-label">reactions</span>
      </div>
    </div>
    <div class="cmd">
      <span>$ </span><code>ssh eipi.boo</code>
    </div>
    <p class="online"><span class="dot"></span>{} online now</p>
    <p class="footer">
      <a href="https://github.com/pwnwriter/eipi.boo">source</a>
    </p>
  </div>
</body>
</html>"#,
        confessions, humans, confessions, humans, replies, reactions, online,
    )
}

fn confession_page(id: i64, text: &str, age: &str, reactions: i64, replies: i64) -> String {
    let truncated: String = text.chars().take(160).collect();
    let og_desc = format!(
        "{} | {} reactions, {} replies",
        truncated, reactions, replies
    );

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>confession #{} | eipi.boo</title>
  <meta property="og:title" content="confession #{} | eipi.boo">
  <meta property="og:description" content="{}">
  <meta property="og:type" content="article">
  <meta property="og:url" content="https://eipi.boo/c/{}">
  <meta name="twitter:card" content="summary">
  <meta name="twitter:title" content="confession #{} | eipi.boo">
  <meta name="twitter:description" content="{}">
  <style>
    * {{ margin: 0; padding: 0; box-sizing: border-box; }}
    body {{
      background: #faf4ed;
      color: #575279;
      font-family: 'Courier New', monospace;
      display: flex;
      justify-content: center;
      align-items: center;
      min-height: 100vh;
    }}
    .card {{
      max-width: 480px;
      width: 90%;
      padding: 2rem;
    }}
    .cloud {{
      background: #f2e9e1;
      border: 2px solid #dfdad9;
      border-radius: 20px;
      padding: 1.5rem 1.8rem;
      position: relative;
      margin-bottom: 1rem;
    }}
    .cloud::after {{
      content: '';
      position: absolute;
      bottom: -12px;
      left: 30px;
      width: 20px;
      height: 20px;
      background: #f2e9e1;
      border: 2px solid #dfdad9;
      border-radius: 50%;
      border-top-color: #f2e9e1;
    }}
    .cloud::before {{
      content: '';
      position: absolute;
      bottom: -22px;
      left: 22px;
      width: 12px;
      height: 12px;
      background: #f2e9e1;
      border: 2px solid #dfdad9;
      border-radius: 50%;
      border-top-color: #f2e9e1;
    }}
    .text {{
      font-size: 1.1rem;
      line-height: 1.6;
      color: #575279;
    }}
    .meta {{
      display: flex;
      justify-content: space-between;
      margin-top: 1.5rem;
      padding-top: 0.5rem;
      color: #9893a5;
      font-size: 0.85rem;
    }}
    .reactions {{
      color: #b4637a;
    }}
    .cta {{
      text-align: center;
      margin-top: 2rem;
    }}
    .cta a {{
      color: #286983;
      text-decoration: none;
      font-size: 0.9rem;
    }}
    .cta a:hover {{
      text-decoration: underline;
    }}
    .cta .cmd {{
      background: #f2e9e1;
      border: 1px solid #dfdad9;
      border-radius: 6px;
      padding: 0.6rem 1.2rem;
      display: inline-block;
      margin-top: 0.5rem;
    }}
    .cta .cmd code {{
      color: #286983;
      font-weight: bold;
    }}
  </style>
</head>
<body>
  <div class="card">
    <div class="cloud">
      <p class="text">{}</p>
    </div>
    <div class="meta">
      <span class="reactions">{} reactions · {} replies</span>
      <span>{}</span>
    </div>
    <div class="cta">
      <p>react and reply over ssh:</p>
      <div class="cmd"><code>$ ssh eipi.boo</code></div>
    </div>
  </div>
</body>
</html>"#,
        id,
        id,
        html_escape(&og_desc),
        id,
        id,
        html_escape(&og_desc),
        html_escape(text),
        reactions,
        replies,
        age,
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

async fn landing(State(state): State<Arc<AppState>>) -> Html<String> {
    let db = state.db.lock();
    let stats = crate::db::stats(&db);
    drop(db);
    let online = state.online.load(std::sync::atomic::Ordering::Relaxed);
    Html(landing_page(
        stats.confessions,
        stats.humans,
        stats.replies,
        stats.reactions,
        online,
    ))
}

async fn confession(Path(id): Path<i64>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let db = state.db.lock();
    let Some(c) = crate::db::get_by_id(&db, id) else {
        drop(db);
        return (
            axum::http::StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
            Html(String::from(
                "<h1>not found</h1><p><a href=\"/\">back to eipi.boo</a></p>",
            )),
        );
    };

    let age = crate::model::confession::time_ago(&c.created_at);
    let reactions: i64 = c.reactions.iter().map(|r| r.count).sum();
    drop(db);

    let page = confession_page(id, &c.text, &age, reactions, c.reply_count);
    (
        axum::http::StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        Html(page),
    )
}

pub async fn serve(state: Arc<AppState>) {
    let http_addr =
        std::env::var("EIPI_HTTP_LISTEN").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    let app = Router::new()
        .route("/", get(landing))
        .route("/c/{id}", get(confession))
        .with_state(state);

    info!("Starting HTTP server on {}", http_addr);

    let listener = match tokio::net::TcpListener::bind(&http_addr).await {
        Ok(l) => l,
        Err(e) => {
            log::warn!("Failed to bind HTTP server on {}: {}", http_addr, e);
            return;
        }
    };

    if let Err(e) = axum::serve(listener, app).await {
        log::warn!("HTTP server error: {}", e);
    }
}
