use std::path::PathBuf;

use camino::Utf8PathBuf;
use serde::Deserialize;

use crate::errors::*;
use crate::message::{Message, MessageType};

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub version: Option<String>,
    pub license: Option<String>,
}

impl ProjectConfig {
    pub fn load(project_root: Option<PathBuf>) -> Result<Option<ProjectConfig>> {
        if let Some(project) = ProjectConfig::get_project(&project_root) {
            match project.kind {
                axo_project::WorkspaceKind::Rust => {
                    Message::new(MessageType::Info, "Detected Rust project...").print();
                    tracing::info!("Detected Rust project...");
                }
                axo_project::WorkspaceKind::Javascript => {
                    Message::new(MessageType::Info, "Detected JavaScript project...").print();
                    tracing::info!("Detected JavaScript project...");
                }
            }

            // FIXME: Oranda currently has no notion of workspaces with multiple binaries,
            // so we just refuse to make progress if we find more than one.

            let mut bin_package = None;
            for (_idx, package) in project.packages() {
                if !package.binaries.is_empty() {
                    if bin_package.is_none() {
                        bin_package = Some(package);
                    } else {
                        Message::new(
                            MessageType::Warning,
                            "Your project has multiple binaries, Oranda doesn't support that",
                        )
                        .print();
                        tracing::warn!(
                            "Your project has multiple binaries, Oranda doesn't support that"
                        );
                        return Ok(None);
                    }
                }
            }

            if let Some(package) = bin_package {
                return Ok(Some(ProjectConfig {
                    name: package.name.clone(),
                    description: package.description.clone().unwrap_or_default(),
                    homepage: package.homepage_url.clone(),
                    repository: package.repository_url.clone(),
                    version: package.version.as_ref().map(|v| v.to_string()),
                    license: package.license.clone(),
                }));
            } else {
                Message::new(
                    MessageType::Warning,
                    "Your project doesn't seem to have binaries",
                )
                .print();
                tracing::warn!("Your project doesn't seem to have binaries");
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_project(project_root: &Option<PathBuf>) -> Option<axo_project::WorkspaceInfo> {
        // Get the general info about the project (via axo-project)
        let start_dir = project_root.clone().unwrap_or_else(|| {
            std::env::current_dir().expect("couldn't get current working dir!?")
        });
        let start_dir = Utf8PathBuf::from_path_buf(start_dir).expect("project path isn't utf8!?");
        let Some(project) = axo_project::get_project(&start_dir) else {
            Message::new(MessageType::Warning, "Could not identify project type...").print();
            tracing::warn!("Could not identify project type...");
            return None;
        };
        Some(project)
    }
}
