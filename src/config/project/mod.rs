use std::path::PathBuf;

use axoproject::{WorkspaceSearch, WorkspaceInfo};
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
                axoproject::WorkspaceKind::Rust => {
                    Message::new(MessageType::Info, "Detected Rust project...").print();
                    tracing::info!("Detected Rust project...");
                }
                axoproject::WorkspaceKind::Javascript => {
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
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Get information about the project workspace (using axoproject)
    pub fn get_project(project_root: &Option<PathBuf>) -> Option<WorkspaceInfo> {
        // Start in the project root, or failing that current dir
        let start_dir = project_root.clone().unwrap_or_else(|| {
            std::env::current_dir().expect("couldn't get current working dir!?")
        });
        let start_dir = Utf8PathBuf::from_path_buf(start_dir).expect("project path isn't utf8!?");
        
        // Clamp the search for project files to the nearest oranda.json, or failing, that cwd.
        // This is overly conservative to give us more options in the future
        //
        // FIXME: this is currently kinda whack because oranda won't actually use the oranda.json file
        // we find here... that choice is made in the caller.
        let clamp_to_dir = if let Ok(file) = axoproject::find_file("oranda.json", &start_dir, None) {
            file.parent().unwrap().to_owned()    
        } else {
            start_dir.clone()
        };

        // Search for workspaces and process the results
        let workspaces = axoproject::get_workspaces(&start_dir, Some(&clamp_to_dir));
        let rust_workspace = Self::handle_search_result(workspaces.rust, "rust");
        let js_workspace = Self::handle_search_result(workspaces.javascript, "javascript");
        
        // Now pick the "best" one based on which one is "deeper" in the filesystem
        // (and therefore closer to the start dir, since we only look at ancestors).
        //
        // This is kinda hacky, but it's a starting point.
        let all_workspaces = vec![rust_workspace, js_workspace];
        let mut best_workspace_depth = 0;
        let mut best_workspace = None;
        for workspace in all_workspaces {
            let Some(workspace) = workspace else {
                continue;
            };
            let workspace_depth = workspace.manifest_path.ancestors().count();
            if workspace_depth > best_workspace_depth {
                best_workspace = Some(workspace);
                best_workspace_depth = workspace_depth;
            }
        }

        best_workspace
    }

    fn handle_search_result(search: WorkspaceSearch, name: &str) -> Option<WorkspaceInfo> {
        match search {
            axoproject::WorkspaceSearch::Found(info) => Some(info),
            axoproject::WorkspaceSearch::Broken { manifest_path, cause } => {
                let warning = OrandaError::BrokenProject {
                    kind: name.to_owned(),
                    manifest_path,
                    cause,
                };
                eprintln!("{:?}", miette::Report::new(warning));
                None
            },
            axoproject::WorkspaceSearch::Missing(cause) => {
                // Just quietly log this in case useful
                tracing::info!("Couldn't find {name} project {:?}", &miette::Report::new(cause));
                None
            },
        }
    }
}
