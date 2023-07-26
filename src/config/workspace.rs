use crate::config::{ApplyLayer, ApplyOptExt, ApplyValExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
/// Configuration regarding multi-project "workspaces".
pub struct WorkspaceLayer {
    /// The top-level name to be used in the index page
    pub name: Option<String>,
    /// Whether to generate an index page linking workspace members together
    pub generate_index: Option<bool>,
    /// A list of workspace members
    pub members: Option<Vec<WorkspaceMember>>,
    /// Whether to enable workspace autodetection
    pub auto: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Hash, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceMember {
    /// Slug for the generated URLs and directories
    pub slug: String,
    /// Path to the workspace member directory
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Clone)]
pub struct WorkspaceConfig {
    pub name: Option<String>,
    pub generate_index: bool,
    pub members: Vec<WorkspaceMember>,
    pub auto: bool,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            name: Some("My Oranda Config".to_string()),
            generate_index: true,
            members: Vec::new(),
            auto: false,
        }
    }
}

impl ApplyLayer for WorkspaceConfig {
    type Layer = WorkspaceLayer;
    fn apply_layer(&mut self, layer: Self::Layer) {
        let WorkspaceLayer {
            name,
            members,
            generate_index,
            auto,
        } = layer;
        self.name.apply_opt(name);
        self.generate_index.apply_val(generate_index);
        self.members.apply_val(members);
        self.auto.apply_val(auto);
    }
}
