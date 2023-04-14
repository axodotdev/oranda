use camino::{Utf8Path, Utf8PathBuf};
use std::net::SocketAddr;

use crate::message::{Message, MessageType};
use oranda::config::Config;
use oranda::errors::*;

use axum::{http::StatusCode, response::Redirect, routing::get, routing::get_service, Router};

use clap::Parser;
use tower_http::services::ServeDir;

#[derive(Debug, Default, Parser)]
pub struct Serve {
    #[arg(long, default_value = "7979")]
    port: u16,
}

impl Serve {
    pub fn new(port: Option<u16>) -> Self {
        Serve {
            port: port.unwrap_or(7979),
        }
    }

    pub fn run(&self) -> Result<()> {
        Message::new(MessageType::Info, "Running serve...").print();
        tracing::info!("Running serve...");
        let config = Config::build(&Utf8PathBuf::from("./oranda.json"))?;
        if Utf8Path::new(&config.dist_dir).is_dir() {
            let msg = format!("Found build in {} directory...", &config.dist_dir);
            Message::new(MessageType::Info, &msg).print();
            if let Some(prefix) = config.path_prefix {
                tracing::debug!("`path_prefix` configured: {}", &prefix);
                self.serve_prefix(&config.dist_dir, &prefix)?;
            } else {
                self.serve(&config.dist_dir)?;
            }
            Ok(())
        } else {
            Err(OrandaError::BuildNotFound {
                dist_dir: config.dist_dir.to_string(),
            })
        }
    }

    #[tokio::main]
    async fn serve(&self, dist_dir: &str) -> Result<()> {
        let serve_dir =
            get_service(ServeDir::new(dist_dir)).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            });

        let app = Router::new().nest_service("/", serve_dir);

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let msg = format!("Your project is available at: http://{}", addr);
        Message::new(MessageType::Success, &msg).print();
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("failed to start server");
        Ok(())
    }

    #[tokio::main]
    async fn serve_prefix(&self, dist_dir: &str, prefix: &str) -> Result<()> {
        let serve_dir =
            get_service(ServeDir::new(dist_dir)).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            });
        let prefix_route = format!("/{}", prefix);
        let oranda_route = format!("/{}/oranda.css", prefix);
        let custom_route = format!("/{}/custom.css", prefix);
        let app = Router::new()
            .nest_service(&prefix_route, serve_dir)
            .route(
                "/oranda.css",
                get(move || async {
                    let oranda_route = oranda_route;
                    Redirect::permanent(&oranda_route)
                }),
            )
            .route(
                "/custom.css",
                get(move || async {
                    let custom_route = custom_route;
                    Redirect::permanent(&custom_route)
                }),
            );

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let msg = format!("Your project is available at: http://{}/{}", addr, prefix);
        Message::new(MessageType::Success, &msg).print();
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("failed to start server");
        Ok(())
    }
}
