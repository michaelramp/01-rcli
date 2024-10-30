use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);

    let state = HttpServeState { path: path.clone() };
    // axum router
    let router = Router::new()
        .nest_service("/tower", ServeDir::new(path))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, Html<String>) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            Html(format!("File {} note found", p.display())),
        )
    } else {
        println!("p is a directory: {:?}", p.is_dir());
        // TODO: test p is a directory
        // if it is a directory, list all files/subdirectories
        // as <li><a href="/path/to/file">file name</a></li>
        // <html><body><ul>...</ul></body></html>
        if p.is_dir() {
            let mut content = String::new();
            content.push_str("<html><body><ul>");
            let entries = tokio::fs::read_dir(&p)
                .await
                .expect("Failed to read directory");
            let mut entries = entries;
            println!("entries: {:?}", entries);
            while let Ok(Some(entry)) = entries.next_entry().await {
                println!("entry: {:?}", entry);
                let path = entry.path();
                let path_display = path.strip_prefix(&state.path).unwrap().display();
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();

                content.push_str(&format!(
                    r#"<li><a href="/{}">{}</a></li>"#,
                    path_display, file_name
                ));
            }
            content.push_str("</ul></body></html>");
            return (StatusCode::OK, Html(content));
        }
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, Html(content))
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Html(e.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.0.trim().starts_with("[package]"));
    }
}
