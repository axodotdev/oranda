use camino::{Utf8Path, Utf8PathBuf};
use std::net::SocketAddr;
use std::sync::mpsc::Receiver;
use std::thread;

use oranda::config::Config;
use oranda::errors::*;

use axum::{http::StatusCode, routing::get_service, Router};

use clap::Parser;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

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
        let config = Self::build_config()?;
        if Utf8Path::new(&config.build.dist_dir).is_dir() {
            self.serve(&config.build.dist_dir, &config.build.path_prefix, None)?;
            Ok(())
        } else {
            Err(OrandaError::BuildNotFound {
                dist_dir: config.build.dist_dir.to_string(),
            })
        }
    }

    pub fn run_with_livereload(&self, rx: Receiver<()>) -> Result<()> {
        let config = Self::build_config()?;
        if Utf8Path::new(&config.build.dist_dir).is_dir() {
            let livereload = LiveReloadLayer::new();
            self.serve(
                &config.build.dist_dir,
                &config.build.path_prefix,
                Some((livereload, rx)),
            )?;

            Ok(())
        } else {
            Err(OrandaError::BuildNotFound {
                dist_dir: config.build.dist_dir.to_string(),
            })
        }
    }

    #[tokio::main]
    async fn serve(
        &self,
        dist_dir: &str,
        path_prefix: &Option<String>,
        livereload: Option<(LiveReloadLayer, Receiver<()>)>,
    ) -> Result<()> {
        let serve_dir =
            get_service(ServeDir::new(dist_dir)).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            });

        let prefix_route = if let Some(prefix) = path_prefix {
            format!("/{}", prefix)
        } else {
            "/".to_string()
        };
        let mut app = Router::new().nest_service(&prefix_route, serve_dir);
        if let Some(livereload) = livereload {
            let (livereload, rx) = livereload;
            let reloader = livereload.reloader();
            app = app.layer(livereload);

            // Because the server will later block this thread, spawn another thread to handle
            // reload request messages.
            thread::spawn(move || loop {
                rx.recv().expect("broken pipe");
                reloader.reload();
            });
        }

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let msg = format!(
            "Your project is available at: http://{}/{}",
            addr,
            path_prefix.as_ref().unwrap_or(&String::new())
        );
        tracing::info!(success = true, "{}", &msg);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("failed to start server");
        Ok(())
    }

    fn build_config() -> Result<Config> {
        let workspace_config_path = &Utf8PathBuf::from("./oranda-workspace.json");
        if workspace_config_path.exists() {
            Config::build(workspace_config_path)
        } else {
            Config::build(&Utf8PathBuf::from("./oranda.json"))
        }
    }
}
