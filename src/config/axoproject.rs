use crate::config::workspace::WorkspaceMember;
use axoproject::{PackageIdx, WorkspaceInfo, WorkspaceSearch};
use camino::{Utf8Path, Utf8PathBuf};
use std::path::PathBuf;

use super::ProjectLayer;
use crate::errors::*;

/// Info gleaned from axoproject
#[derive(Debug)]
pub struct AxoprojectLayer {
    /// Generic project info
    pub project: Option<ProjectLayer>,
    /// Did they have cargo_dist settings?
    pub cargo_dist: Option<bool>,
    /// Information about workspace packages
    pub members: Option<Vec<WorkspaceMember>>,
}

impl AxoprojectLayer {
    /// Load package information about a single-package workspace, where the package is equal to the
    /// current working dir. This is in opposition to our workspace support, which needs to be
    /// explicitly enabled. axoproject is workspace-aware, but we don't use the multi-package
    /// workspace functionality it gives us when ran like this.
    pub fn load(project_root: Option<PathBuf>) -> Result<Option<AxoprojectLayer>> {
        // Start in the project root, or failing that current dir
        let start_dir = project_root.unwrap_or_else(|| {
            std::env::current_dir().expect("couldn't get current working dir!?")
        });
        let start_dir = Utf8PathBuf::from_path_buf(start_dir).expect("project path isn't utf8!?");

        let workspace = Self::get_best_workspace(&start_dir);
        let Some(workspace) = workspace else {
            return Ok(None);
        };
        let project = Self::get_root_package(&start_dir, workspace);

        if let Some((workspace, pkg)) = project {
            // Cool we found the best possible match, now extract all the values we care about from it
            let package = workspace.package(pkg);

            // If there's a [workspace.metadata.dist] table, we can auto-enable cargo-dist
            // If there's no [workspace.metadata] table at all, inconclusive.
            let cargo_dist = workspace
                .cargo_metadata_table
                .as_ref()
                .map(|t| t.get("dist").is_some());
            Ok(Some(AxoprojectLayer {
                project: Some(ProjectLayer {
                    name: Some(package.name.clone()),
                    description: package.description.clone(),
                    homepage: package.homepage_url.clone(),
                    repository: package.repository_url.clone(),
                    version: package.version.as_ref().map(|v| v.to_string()),
                    license: package.license.clone(),
                    readme_path: package.readme_file.as_ref().map(|v| v.to_string()),
                }),
                cargo_dist,
                members: None,
            }))
        } else {
            Ok(None)
        }
    }

    /// Load packages from an actual workspace. This is in contract to `load`, which only collects
    /// information about one package. Here, we simply collect workspace metadata for every
    /// found workspace member.
    pub fn load_workspace(project_root: &Utf8Path) -> Result<Option<AxoprojectLayer>> {
        // Just ignore the package this function picks out for us. We want all packages instead
        let Some(workspace) = Self::get_best_workspace(project_root) else {
            return Ok(None);
        };

        // Gimme all packages!
        let mut members = Vec::new();
        for (_, package) in workspace.packages() {
            let member = WorkspaceMember {
                path: package.package_root.clone().into(),
                slug: slug::slugify(package.name.clone()),
            };
            members.push(member);
        }

        Ok(Some(AxoprojectLayer {
            project: None,
            cargo_dist: None,
            members: Some(members),
        }))
    }

    /// Given context, fetches workspaces and returns whichever "wins". Right now, this means Cargo
    /// projects always win over JS projects, but this will change in the future as we introduce more
    /// criteria.
    pub fn get_best_workspace(start_dir: &Utf8Path) -> Option<WorkspaceInfo> {
        // Clamp the search for project files to the the start dir, because oranda
        // wants to work in so many different situations that things get muddy very quickly
        let clamp_to_dir = start_dir;

        // Search for workspaces and process the results
        let workspaces = axoproject::get_workspaces(start_dir, Some(clamp_to_dir));
        let rust_workspace = Self::handle_search_result(workspaces.rust, "rust");
        let js_workspace = Self::handle_search_result(workspaces.javascript, "javascript");

        // Now pick the "best" one based on... absolutely nothing right now! Since we clamp to
        // one dir, all the parseable projects are on perfectly even footing, so we just
        // will always pick the Cargo.toml over the package.json. In the future we'll have
        // configs to disambiguate.
        let all_workspaces = vec![rust_workspace, js_workspace];
        let mut best_workspace: Option<WorkspaceInfo> = None;
        let mut rejected_workspaces = vec![];
        for workspace in all_workspaces {
            let Some(workspace) = workspace else {
                continue;
            };

            // In the future this will be some more complex criteria like "closes package" or
            // "has an oranda config", but for now the criteria is "first one wins".
            let is_better = best_workspace.is_none();
            if is_better {
                if let Some(defeated) = best_workspace {
                    rejected_workspaces.push(defeated);
                }
                best_workspace = Some(workspace);
            } else {
                rejected_workspaces.push(workspace);
            }
        }

        // Warn about the existence of perfectly good losers
        for reject_ws in rejected_workspaces {
            let message = format!(
                "Also found a {:?} project at {}, but we're ignoring it",
                reject_ws.kind, reject_ws.manifest_path,
            );
            tracing::warn!("{}", &message);
        }

        best_workspace
    }

    /// Handles the `WorkspaceSearch` enum, emitting warnings for a bunch of cases.
    fn handle_search_result(search: WorkspaceSearch, name: &str) -> Option<WorkspaceInfo> {
        match search {
            WorkspaceSearch::Found(workspace) => Some(workspace),
            WorkspaceSearch::Broken {
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
            WorkspaceSearch::Missing(cause) => {
                // Just quietly debug log this in case it's useful
                tracing::trace!(
                    "Couldn't find a {name} project: {:?}",
                    &miette::Report::new(cause)
                );
                None
            }
        }
    }

    /// Given a workspace, tries to find the "root" package that's contained in the same directory
    /// as the workspace root.
    fn get_root_package(
        start_dir: &Utf8Path,
        workspace: WorkspaceInfo,
    ) -> Option<(WorkspaceInfo, PackageIdx)> {
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

        package.map(|pkg_idx| (workspace, pkg_idx))
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
