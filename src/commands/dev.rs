use std::path::PathBuf;

use axoproject::WorkspaceSearch;
use camino::Utf8PathBuf;
use clap::Parser;
use notify::{event::ModifyKind, EventKind, Watcher};

use crate::{
    commands::{Build, Serve},
    message::{Message, MessageType},
};
use oranda::{config::Config, errors::*};

#[derive(Clone, Debug, Parser)]
pub struct Dev {
    /// The port for the file server to be launched on
    #[arg(long)]
    port: Option<u16>,
    /// The project root we want to build from
    #[arg(long)]
    project_root: Option<Utf8PathBuf>,
    /// Custom path to an oranda configuration file
    #[arg(long)]
    config_path: Option<Utf8PathBuf>,
    /// Skip the first build before starting to watch for changes
    #[arg(long)]
    no_first_build: bool,
    /// List of extra paths to watch
    #[arg(short, long)]
    include_paths: Option<Vec<Utf8PathBuf>>,
}

impl Dev {
    pub fn run(self) -> Result<()> {
        Message::new(
            MessageType::Info,
            "Starting dev, looking for paths to watch...",
        )
        .print();
        tracing::info!("Starting dev, looking for paths to watch...");

        let config = Config::build(
            &self
                .config_path
                .clone()
                .unwrap_or(Utf8PathBuf::from("./oranda.json")),
        )?;
        let mut paths_to_watch = vec![];
        // Watch for the readme file
        paths_to_watch.push(config.readme_path);
        // Watch for the oranda config file
        paths_to_watch.push(
            self.config_path
                .clone()
                .unwrap_or(Utf8PathBuf::from("./oranda.json"))
                .into(),
        );

        // Watch for any user-provided paths
        if self.include_paths.is_some() {
            let mut include_paths: Vec<String> = self
                .include_paths
                .unwrap()
                .iter()
                .map(|p| p.to_string())
                .collect();
            paths_to_watch.append(&mut include_paths);
        }

        // Watch for additional pages, if we have any
        if config.additional_pages.is_some() {
            let mut additional_pages: Vec<String> =
                config.additional_pages.unwrap().values().cloned().collect();
            paths_to_watch.append(&mut additional_pages);
        }

        // Watch for the mdbook directory, if we have it
        if config.mdbook.is_some() {
            // FIXME: We generate the mdbook html content in a subfolder of this folder, which means we can't watch
            // the folder recursively with `notify`. This breaks usage for users who use a nested mdbook docs structure,
            // and it's something we should handle.
            paths_to_watch.push(config.mdbook.unwrap().path);
        }

        // Watch for any project manifest files
        let project = axoproject::get_workspaces("./".into(), None);
        if let WorkspaceSearch::Found(workspace) = project.rust {
            paths_to_watch.push(workspace.manifest_path.into());
        }
        if let WorkspaceSearch::Found(workspace) = project.javascript {
            paths_to_watch.push(workspace.manifest_path.into());
        }

        Message::new(
            MessageType::Info,
            &format!(
                "Found {} paths to watch, starting watch...",
                paths_to_watch.len()
            ),
        )
        .print();
        tracing::info!(
            "Found {} paths to watch, starting watch...",
            paths_to_watch.len()
        );
        Message::new(
            MessageType::Debug,
            &format!("Files watched: {:?}", paths_to_watch),
        )
        .print();

        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::RecommendedWatcher::new(tx, notify::Config::default())?;

        for path in paths_to_watch {
            let path = PathBuf::from(path);
            watcher.watch(path.as_path(), notify::RecursiveMode::NonRecursive)?;
        }

        if !self.no_first_build {
            Build::new(self.project_root.clone(), self.config_path.clone()).run()?;
        }
        // Spawn the serve process out into a separate thread so that we can loop through received events on this thread
        let _ = std::thread::spawn(move || Serve::new(self.port).run());
        for res in rx {
            match res {
                Ok(event) => {
                    // We only care about content or name changes, not metadata
                    if let EventKind::Modify(ModifyKind::Metadata(_)) = event.kind {
                        continue;
                    }
                    Message::new(
                        MessageType::Info,
                        &format!("Path(s) {:?} changed, rebuilding...", event.paths),
                    )
                    .print();

                    Build::new(self.project_root.clone(), self.config_path.clone())
                        .run()
                        .unwrap();
                }
                Err(err) => {
                    // FIXME: Do we want to do something other than panicking?
                    panic!("Error while watching for filesystem changes: {:?}", err);
                }
            }
        }
        Ok(())
    }
}
