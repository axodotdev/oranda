use crate::config::Config;
use crate::errors::*;
use axum::{http::StatusCode, routing::get_service, Router};
use axum_extra::routing::SpaRouter;
use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::services::ServeDir;

#[derive(Debug, Parser)]
pub struct Serve {
    #[arg(long, default_value = "7979")]
    port: u16,
    #[arg(long, default_value = "./oranda.json")]
    config: PathBuf,
}

impl Serve {
    pub fn run(&self) -> Result<()> {
        let config = Config::build(&self.config)?;
        self.serve(config)?;
        Ok(())
    }

    #[tokio::main]
    async fn serve(&self, config: Config) -> Result<()> {
        let route = if let Some(path_prefix) = config.path_prefix {
            format!("/{}", path_prefix)
        } else {
            String::from("/")
        };

        let app = Router::new().merge(
            SpaRouter::new(route.as_str(), config.dist_dir)
                .index_file("index.html")
                .handle_error(|error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                }),
        );

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        println!("listening on http://{}{}", addr, route);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("failed to start server");
        Ok(())
    }
}
