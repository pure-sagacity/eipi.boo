mod handler;
pub(crate) mod input;

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use parking_lot::Mutex;
use tokio::sync::broadcast;

use anyhow::Result;
use log::info;
use russh::server::{self, Server as _};

use crate::db;
use handler::ClientHandler;

pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
    pub online: AtomicUsize,
    pub notify: broadcast::Sender<()>,
}

struct SshServer {
    state: Arc<AppState>,
}

impl server::Server for SshServer {
    type Handler = ClientHandler;

    fn new_client(&mut self, addr: Option<std::net::SocketAddr>) -> ClientHandler {
        info!("New connection from {:?}", addr);
        ClientHandler::new(self.state.clone())
    }
}

fn load_or_generate_host_key(path: &str) -> Result<russh::keys::PrivateKey> {
    if Path::new(path).exists() {
        info!("Loading host key from {}", path);
        let key = russh::keys::load_secret_key(path, None)?;
        Ok(key)
    } else {
        info!("Generating new Ed25519 host key at {}", path);
        let key =
            russh::keys::PrivateKey::random(&mut rand::rng(), russh::keys::Algorithm::Ed25519)
                .map_err(|e| anyhow::anyhow!("Failed to generate key: {}", e))?;

        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        key.write_openssh_file(Path::new(path), russh::keys::ssh_key::LineEnding::LF)
            .map_err(|e| anyhow::anyhow!("Failed to write host key: {}", e))?;

        Ok(key)
    }
}

pub async fn run() -> Result<()> {
    let db_path = std::env::var("EIPI_DB_PATH").unwrap_or_else(|_| "eipi.db".to_string());
    let host_key_path =
        std::env::var("EIPI_HOST_KEY").unwrap_or_else(|_| "assets/host_key".to_string());
    let listen_addr = std::env::var("EIPI_LISTEN").unwrap_or_else(|_| "0.0.0.0:22".to_string());

    let conn = db::init(&db_path)?;
    let (notify_tx, _) = broadcast::channel(16);
    let state = Arc::new(AppState {
        db: Mutex::new(conn),
        online: AtomicUsize::new(0),
        notify: notify_tx,
    });

    let host_key = load_or_generate_host_key(&host_key_path)?;

    let config = Arc::new(server::Config {
        keys: vec![host_key],
        ..Default::default()
    });

    // spawn HTTP server for shareable confession links
    tokio::spawn(crate::helper::web::serve(state.clone()));

    info!("Starting eipi.boo SSH server on {}", listen_addr);
    info!(
        "Connect with: ssh -p {} localhost",
        listen_addr.rsplit(':').next().unwrap_or("22")
    );

    let mut server = SshServer { state };
    server.run_on_address(config, &listen_addr).await?;

    Ok(())
}
