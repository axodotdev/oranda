use std::net::SocketAddr;

use crate::config::Config;
use crate::errors::*;
use crate::message::{self, MessageType};
use axum::{http::StatusCode, routing::get_service, Router};
use clap::Parser;
use tower_http::services::ServeDir;
use tracing;

#[derive(Debug, Parser)]
pub struct Serve {
    #[arg(long, default_value = "7979")]
    port: u16,
}

impl Serve {
    pub fn run(&self) -> Result<()> {
        tracing::info!("{}", message::build(MessageType::Info, "Running serve..."));
        let config = Config::build()?;
        self.serve(config)?;
        Ok(())
    }

    #[tokio::main]
    async fn serve(&self, config: Config) -> Result<()> {
        let static_files_service = get_service(ServeDir::new(config.dist_dir)).handle_error(
            |error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            },
        );

        let app = Router::new().fallback(static_files_service);

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let msg = format!("Listening on {}...", addr);
        tracing::info!("{}", message::build(MessageType::Info, &msg));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("failed to start server");
        Ok(())
    }
}
