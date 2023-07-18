use crate::config::{ApplyLayer, ApplyOptExt, ApplyValExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, JsonSchema)]
/// Configuration regarding multi-project "workspaces".
pub struct WorkspaceLayer {
    /// The top-level name to be used in the index page
    pub name: Option<String>,
    /// A list of workspace members
    pub members: Option<Vec<WorkspaceMember>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct WorkspaceMember {
    /// Slug for the generated URLs and directories
    pub slug: String,
    /// Path to the workspace member directory
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Clone)]
pub struct WorkspaceConfig {
    pub name: Option<String>,
    pub members: Vec<WorkspaceMember>,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            name: None,
            members: Vec::new(),
        }
    }
}

impl ApplyLayer for WorkspaceConfig {
    type Layer = WorkspaceLayer;
    fn apply_layer(&mut self, layer: Self::Layer) {
        let WorkspaceLayer { name, members } = layer;
        self.name.apply_opt(name);
        self.members.apply_val(members);
    }
}
