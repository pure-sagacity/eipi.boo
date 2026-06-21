use log::{info, warn};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const PAGE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>eipi.boo</title>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body {
      background: #faf4ed;
      color: #575279;
      font-family: 'Courier New', monospace;
      display: flex;
      justify-content: center;
      align-items: center;
      min-height: 100vh;
    }
    .container {
      text-align: center;
      padding: 2rem;
    }
    h1 {
      font-size: 3rem;
      color: #b4637a;
      margin-bottom: 0.5rem;
    }
    .tagline {
      color: #9893a5;
      font-size: 1.1rem;
      margin-bottom: 2.5rem;
    }
    .cmd {
      background: #f2e9e1;
      border: 1px solid #dfdad9;
      border-radius: 8px;
      padding: 1.2rem 2rem;
      display: inline-block;
      margin-bottom: 2rem;
    }
    .cmd span {
      color: #56949f;
      font-size: 1.3rem;
    }
    .cmd code {
      color: #286983;
      font-size: 1.3rem;
      font-weight: bold;
    }
    .footer {
      color: #9893a5;
      font-size: 0.85rem;
    }
    .footer a {
      color: #907aa9;
      text-decoration: none;
    }
    .footer a:hover {
      text-decoration: underline;
    }
  </style>
</head>
<body>
  <div class="container">
    <h1>eipi.boo</h1>
    <p class="tagline">anonymous confessions over ssh</p>
    <div class="cmd">
      <span>$ </span><code>ssh eipi.boo</code>
    </div>
    <p class="footer">
      <a href="https://github.com/pwnwriter/eipi.boo">source</a>
    </p>
  </div>
</body>
</html>"#;

pub async fn serve() {
    let addr = std::env::var("EIPI_WEB_LISTEN").unwrap_or_else(|_| "0.0.0.0:80".to_string());

    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            warn!("Web server failed to bind on {}: {}", addr, e);
            return;
        }
    };

    info!("Web server listening on {}", addr);

    loop {
        let Ok((mut stream, _)) = listener.accept().await else {
            continue;
        };

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            let _ = stream.read(&mut buf).await;

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                PAGE.len(),
                PAGE
            );
            let _ = stream.write_all(response.as_bytes()).await;
            let _ = stream.shutdown().await;
        });
    }
}
