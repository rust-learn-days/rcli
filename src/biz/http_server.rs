use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::{header, Response, StatusCode};
use axum::routing::get;
use axum::Router;
use log::{info, warn};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Debug)]
struct HttpServerState {
    path: PathBuf,
}

pub async fn http_server(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {} on http://{}", path.display(), addr);

    let state = HttpServerState { path: path.clone() };
    let service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_zstd()
        .precompressed_deflate();
    // axum router
    let router = Router::new()
        //http://localhost:8080// visit root index
        .route("/*path", get(file_handler))
        .nest_service("/tower", service)
        .with_state(Arc::new(state));

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServerState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, Infallible> {
    let full_path = std::path::Path::new(&state.path).join(path);
    if !full_path.exists() {
        warn!("Not found {}", full_path.display());
        // (StatusCode::NOT_FOUND, "Not found file".to_string())
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not found file".to_string())
            .unwrap())
    } else {
        if full_path.is_dir() || full_path == std::path::Path::new("/") {
            let mut new_path = full_path.clone();
            if new_path == std::path::Path::new("/") {
                new_path = PathBuf::from(std::path::Path::new("./"))
            }
            //遍历下游文件，并输出一个index.html显示目录文件
            let mut content = String::new();
            content.push_str("<html><body><ul>");
            for entry in std::fs::read_dir(new_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                let name = entry.file_name();
                content.push_str(&format!(
                    "<li><a href=\"{}\">{}</a></li>",
                    path.to_string_lossy(),
                    name.to_string_lossy()
                ));
            }
            content.push_str("</ul></body></html>");
            return Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/html")
                .body(content)
                .unwrap());
        }
        info!("Serving {}", full_path.display());
        match std::fs::read(full_path) {
            Ok(content) => Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/html")
                .body(String::from_utf8_lossy(&content).to_string())
                .unwrap()),
            Err(e) => {
                warn!("Error reading file: {}", e);
                Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(format!("Error reading file: {}", e).to_string())
                    .unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler_not_found() {
        let state = Arc::new(HttpServerState {
            path: PathBuf::from("."),
        });
        let path = Path("test.txt".to_string());
        let s = file_handler(State(state), path).await.unwrap();
        assert_eq!(s.status(), StatusCode::NOT_FOUND);
        assert_eq!(s.body(), "Not found file");
    }

    #[tokio::test]
    async fn test_file_handler_found() {
        let state = Arc::new(HttpServerState {
            path: PathBuf::from("."),
        });
        let path = Path("Cargo.toml".to_string());
        let s = file_handler(State(state), path).await.unwrap();
        assert_eq!(s.status(), StatusCode::OK);
        assert!(s.body().contains("[package]"));
    }

    #[tokio::test]
    async fn test_file_handler_dir() {
        let state = Arc::new(HttpServerState {
            path: PathBuf::from("."),
        });
        let path = Path("/".to_string());
        let s = file_handler(State(state), path).await.unwrap();
        assert_eq!(s.status(), StatusCode::OK);
        assert!(s.body().contains("html"));
    }
}
