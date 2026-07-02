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

fn build_ascii_card(
    text: &str,
    age: &str,
    love: i64,
    replies: i64,
    reactions: i64,
    position: &str,
) -> String {
    let iw: usize = 46;
    let wrapped = crate::model::confession::wrap_text(text, iw - 4);

    let mut lines: Vec<String> = Vec::new();

    // top border
    lines.push(format!("╭{}╮", "─".repeat(iw)));

    // dots + age
    let age_display = format!(" {} ", age);
    let dots = "  ● ● ●";
    let dots_pad = iw.saturating_sub(7 + age_display.len());
    lines.push(format!("│{}{}{}│", dots, " ".repeat(dots_pad), age_display));

    // separator
    lines.push(format!("│{}│", "─".repeat(iw)));

    // empty line
    lines.push(format!("│{}│", " ".repeat(iw)));

    // text lines
    for line in &wrapped {
        let dcols = line.chars().count();
        let right_pad = iw.saturating_sub(dcols + 2);
        lines.push(format!(
            "│  {}{}│",
            html_escape(line),
            " ".repeat(right_pad)
        ));
    }

    // empty line
    lines.push(format!("│{}│", " ".repeat(iw)));

    // separator
    lines.push(format!("│{}│", "─".repeat(iw)));

    // footer
    let mut footer_left = String::from("  ");
    let mut footer_left_len: usize = 2;
    if love > 0 {
        footer_left.push_str(&format!("♥ {}", love));
        footer_left_len += 2 + love.to_string().len();
    }
    if replies > 0 {
        footer_left.push_str(&format!("  \u{F0367} {}", replies));
        footer_left_len += 4 + replies.to_string().len();
    }
    if reactions > 0 {
        footer_left.push_str(&format!("  ✦ {}", reactions));
        footer_left_len += 4 + reactions.to_string().len();
    }
    let pos_part = format!("{}  ", position);
    let footer_pad = iw.saturating_sub(footer_left_len + pos_part.len());
    lines.push(format!(
        "│{}{}{}│",
        footer_left,
        " ".repeat(footer_pad),
        pos_part
    ));

    // bottom border with stem
    let mid = iw / 2;
    lines.push(format!(
        "╰{}┬{}╯",
        "─".repeat(mid),
        "─".repeat(iw - mid - 1)
    ));

    // ascii man
    let center = mid + 1;
    let face = match text.len() {
        0..70 => "\\(^_^)/",
        70..150 => "\\(o_o)/",
        150..220 => "\\(>_<)/",
        _ => "\\(x_x)/",
    };
    lines.push(format!("{}│", " ".repeat(center)));
    lines.push(format!("{}{}", " ".repeat(center.saturating_sub(3)), face));
    lines.push(format!("{}│", " ".repeat(center)));
    lines.push(format!("{}/ \\", " ".repeat(center.saturating_sub(1))));

    lines.join("\n")
}

fn confession_page(
    id: i64,
    text: &str,
    age: &str,
    love: i64,
    reactions: i64,
    replies: i64,
    total: i64,
) -> String {
    let truncated: String = text.chars().take(160).collect();
    let og_desc = format!(
        "{} | {} reactions, {} replies",
        truncated, reactions, replies
    );
    let position = format!("{}/{}", id, total);
    let ascii_card = build_ascii_card(text, age, love, replies, reactions, &position);

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
      flex-direction: column;
      justify-content: center;
      align-items: center;
      min-height: 100vh;
    }}
    .ascii {{
      white-space: pre;
      font-size: 14px;
      line-height: 1.4;
      color: #575279;
    }}
    .ascii .border {{ color: #9893a5; }}
    .ascii .dots-red {{ color: #b4637a; }}
    .ascii .dots-yellow {{ color: #ea9d34; }}
    .ascii .dots-green {{ color: #56949f; }}
    .ascii .age {{ color: #9893a5; }}
    .ascii .heart {{ color: #b4637a; }}
    .ascii .man {{ color: #9893a5; }}
    .cta {{
      text-align: center;
      margin-top: 2rem;
    }}
    .cta .cmd {{
      background: #f2e9e1;
      border: 1px solid #dfdad9;
      border-radius: 6px;
      padding: 0.6rem 1.2rem;
      display: inline-block;
    }}
    .cta .cmd code {{
      color: #286983;
      font-weight: bold;
    }}
    .save {{
      margin-top: 1rem;
    }}
    .save button {{
      background: #f2e9e1;
      border: 1px solid #dfdad9;
      border-radius: 6px;
      padding: 0.5rem 1rem;
      color: #575279;
      font-family: 'Courier New', monospace;
      font-size: 0.85rem;
      cursor: pointer;
    }}
    .save button:hover {{
      background: #dfdad9;
    }}
  </style>
</head>
<body>
  <pre class="ascii" id="card">{}</pre>
  <div class="cta">
    <p style="color: #9893a5; font-size: 0.85rem; margin-bottom: 0.5rem;">react and reply over ssh</p>
    <div class="cmd"><code>$ ssh eipi.boo</code></div>
  </div>
  <div class="save">
    <button onclick="saveImage()">save as image</button>
  </div>
  <canvas id="canvas" style="display:none;"></canvas>
  <script>
  function saveImage() {{
    const pre = document.getElementById('card');
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    const lines = pre.textContent.split('\n');
    const fontSize = 14;
    const lineHeight = fontSize * 1.4;
    const padding = 40;
    ctx.font = fontSize + 'px Courier New, monospace';
    let maxWidth = 0;
    for (const line of lines) {{
      const w = ctx.measureText(line).width;
      if (w > maxWidth) maxWidth = w;
    }}
    canvas.width = maxWidth + padding * 2;
    canvas.height = lines.length * lineHeight + padding * 2;
    ctx.fillStyle = '#faf4ed';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    ctx.font = fontSize + 'px Courier New, monospace';
    ctx.fillStyle = '#575279';
    ctx.textBaseline = 'top';
    for (let i = 0; i < lines.length; i++) {{
      ctx.fillText(lines[i], padding, padding + i * lineHeight);
    }}
    ctx.fillStyle = '#9893a5';
    ctx.font = '12px Courier New, monospace';
    ctx.fillText('eipi.boo', canvas.width - padding - ctx.measureText('eipi.boo').width, canvas.height - 24);
    const link = document.createElement('a');
    link.download = 'confession-{}.png';
    link.href = canvas.toDataURL('image/png');
    link.click();
  }}
  </script>
</body>
</html>"#,
        id,
        id,
        html_escape(&og_desc),
        id,
        id,
        html_escape(&og_desc),
        ascii_card,
        id,
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
    let love = crate::model::confession::love_reactions(&c);
    let stats = crate::db::stats(&db);
    drop(db);

    let page = confession_page(
        id,
        &c.text,
        &age,
        love,
        reactions,
        c.reply_count,
        stats.confessions,
    );
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
