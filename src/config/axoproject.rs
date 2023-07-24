use axoproject::{PackageIdx, WorkspaceInfo, WorkspaceSearch};
use camino::{Utf8Path, Utf8PathBuf};
use std::path::PathBuf;

use super::ProjectLayer;
use crate::errors::*;

/// Info gleaned from axoproject
#[derive(Debug)]
pub struct AxoprojectLayer {
    /// Generic project info
    pub project: ProjectLayer,
    /// Did they have cargo_dist settings?
    pub cargo_dist: Option<bool>,
}

impl AxoprojectLayer {
    pub fn load(project_root: Option<PathBuf>) -> Result<Option<AxoprojectLayer>> {
        // Start in the project root, or failing that current dir
        let start_dir = project_root.unwrap_or_else(|| {
            std::env::current_dir().expect("couldn't get current working dir!?")
        });
        let start_dir = Utf8PathBuf::from_path_buf(start_dir).expect("project path isn't utf8!?");

        if let Some((workspace, pkg)) = AxoprojectLayer::get_project(&start_dir) {
            // Cool we found the best possible match, now extract all the values we care about from it
            let package = workspace.package(pkg);

            // If there's a [workspace.metadata.dist] table, we can auto-enable cargo-dist
            // If there's no [workspace.metadata] table at all, inconclusive.
            let cargo_dist = workspace
                .cargo_metadata_table
                .as_ref()
                .map(|t| t.get("dist").is_some());
            Ok(Some(AxoprojectLayer {
                project: ProjectLayer {
                    name: Some(package.name.clone()),
                    description: package.description.clone(),
                    homepage: package.homepage_url.clone(),
                    repository: package.repository_url.clone(),
                    version: package.version.as_ref().map(|v| v.to_string()),
                    license: package.license.clone(),
                    readme_path: package.readme_file.as_ref().map(|v| v.to_string()),
                },
                cargo_dist,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get information about the project workspace (using axoproject)
    ///
    /// The returned value is info about a Workspace and the specific package in that
    /// workspace that "owns" the start_dir.
    ///
    /// Various warnings will be emitted for situations that Almost Match but are
    /// rejected for one reason or another.
    pub fn get_project(start_dir: &Utf8Path) -> Option<(WorkspaceInfo, PackageIdx)> {
        // Clamp the search for project files to the the start dir, because oranda
        // wants to work in so many different situations that things get muddy very quickly
        let clamp_to_dir = start_dir;

        // Search for workspaces and process the results
        let workspaces = axoproject::get_workspaces(start_dir, Some(clamp_to_dir));
        let rust_workspace = Self::handle_search_result(start_dir, workspaces.rust, "rust");
        let js_workspace =
            Self::handle_search_result(start_dir, workspaces.javascript, "javascript");

        // Now pick the "best" one based on... absolutely nothing right now! Since we clamp to
        // one dir, all the parseable projects are on perfectly even footing, so we just
        // will always pick the Cargo.toml over the package.json. In the future we'll have
        // configs to disambiguate.
        let all_workspaces = vec![rust_workspace, js_workspace];
        let mut best_workspace: Option<(WorkspaceInfo, PackageIdx)> = None;
        let mut rejected_workspaces = vec![];
        for workspace in all_workspaces {
            let Some((workspace, pkg_idx)) = workspace else {
                continue;
            };
            // In the future this will be some more complex criteria
            // like "closest package" or "has an oranda config"
            // For now the criteria is "first one wins"
            let is_better = best_workspace.is_none();
            if is_better {
                if let Some(defeated) = best_workspace {
                    rejected_workspaces.push(defeated);
                }
                best_workspace = Some((workspace, pkg_idx));
            } else {
                rejected_workspaces.push((workspace, pkg_idx));
            }
        }

        if let Some((_, _best_pkg)) = &best_workspace {
            // Report the winner
            // let message = format!("Detected {:?} project...", best_ws.kind);
            // Message::new(MessageType::Info, &message).print();
            // tracing::info!("{}", message);

            // Warn about the existence of perfectly good losers
            for (reject_ws, reject_pkg) in rejected_workspaces {
                let reject_pkg = reject_ws.package(reject_pkg);
                let message = format!(
                    "Also found a {:?} project at {}, but we're ignoring it",
                    reject_ws.kind, reject_pkg.manifest_path
                );
                tracing::warn!("{}", &message);
            }
        }

        best_workspace
    }

    /// Process the raw result of axoproject to print warnings and choose the actual
    /// package in the workspace that applies.
    fn handle_search_result(
        start_dir: &Utf8Path,
        search: WorkspaceSearch,
        name: &str,
    ) -> Option<(WorkspaceInfo, PackageIdx)> {
        match search {
            axoproject::WorkspaceSearch::Found(workspace) => {
                // Now that we found the workspace, find the actual package that appears
                // in the dir we're looking at. We need to use canonicalize here because
                // something in guppy/cargo is desugarring symlinks in their output, so
                // we need to too.
                let package = workspace.packages().find_map(|(idx, p)| {
                    let package_dir = p
                        .manifest_path
                        .parent()
                        .expect("project manifest file wasn't in a dir!?");
                    if is_same_path(package_dir, start_dir) {
                        Some(idx)
                    } else {
                        None
                    }
                });

                if let Some(pkg_idx) = package {
                    // Nice, this package is a perfect candidate
                    Some((workspace, pkg_idx))
                } else {
                    // Found a workspace but none of the packages specifically control this dir.
                    // This can happen if you run oranda in a dir with a virtual Cargo.toml.
                    tracing::warn!("Ignoring {:?} project, oranda is currently per-package and this looks like a whole workspace", workspace.kind);
                    None
                }
            }
            axoproject::WorkspaceSearch::Broken {
                manifest_path,
                cause,
            } => {
                let warning = OrandaError::BrokenProject {
                    kind: name.to_owned(),
                    manifest_path,
                    cause,
                };
                eprintln!("{:?}", miette::Report::new(warning));
                None
            }
            axoproject::WorkspaceSearch::Missing(cause) => {
                // Just quietly log this in case it's useful
                tracing::debug!(
                    "Couldn't find a {name} project: {:?}",
                    &miette::Report::new(cause)
                );
                None
            }
        }
    }
}

fn is_same_path(path1: &Utf8Path, path2: &Utf8Path) -> bool {
    if let Ok(path1) = std::fs::canonicalize(path1) {
        if let Ok(path2) = std::fs::canonicalize(path2) {
            return path1 == path2;
        }
    }
    path1 == path2
}
