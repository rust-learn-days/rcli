use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use log::{info, warn};
use tokio::net::TcpListener;

#[derive(Debug)]
struct HttpServerState {
    path: PathBuf,
}

pub async fn http_server(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {} on http://{}", path.display(), addr);

    let state = HttpServerState { path };

    // axum router
    let router = Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServerState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let full_path = std::path::Path::new(&state.path).join(path);
    if !full_path.exists() {
        warn!("Not found {}", full_path.display());
        (StatusCode::NOT_FOUND, "Not found file".to_string())
    } else {
        info!("Serving {}", full_path.display());
        match std::fs::read(full_path) {
            Ok(content) => (
                StatusCode::OK,
                String::from_utf8_lossy(&content).to_string(),
            ),
            Err(e) => {
                warn!("Error reading file: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error reading file".to_string(),
                )
            }
        }
    }
}
