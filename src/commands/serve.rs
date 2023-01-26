use std::net::SocketAddr;
use std::path::Path;

use oranda::config::Config;
use oranda::errors::*;

use axum::{http::StatusCode, routing::get_service, Router};
use clap::Parser;
use tower_http::services::ServeDir;

#[derive(Debug, Parser)]
pub struct Serve {
    #[arg(long, default_value = "7979")]
    port: u16,
}

impl Serve {
    pub fn run(&self) -> Result<()> {
        let config = Config::build(Path::new("./oranda.json"))?;
        if let Some(prefix) = config.path_prefix {
            self.serve_prefix(&config.dist_dir, &prefix)?;
        } else {
            self.serve(&config.dist_dir)?;
        }
        Ok(())
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
        println!("listening on http://{}", addr);
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
        let app = Router::new().nest_service(&prefix_route, serve_dir);

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        println!("listening on http://{}/{}", addr, prefix);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("failed to start server");
        Ok(())
    }
}
