use crate::config::{ApplyLayer, ApplyOptExt, ApplyValExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
/// Configuration regarding multi-project "workspaces".
pub struct WorkspaceLayer {
    /// The top-level name to be used in the index page
    pub name: Option<String>,
    /// Whether to generate an index page linking workspace members together
    pub generate_index: Option<bool>,
    /// A list of workspace members
    pub members: Option<Vec<WorkspaceMember>>,
    /// A list of members given priority in display
    pub preferred_members: Option<Vec<String>>,
    /// Whether to enable workspace autodetection
    pub auto: Option<bool>,
    /// The path to additional documentation to render
    pub docs_path: Option<String>,
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
    pub preferred_members: Vec<String>,
    pub auto: bool,
    pub docs_path: Option<String>,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            name: Some("My Oranda Config".to_string()),
            generate_index: true,
            members: Vec::new(),
            preferred_members: Vec::new(),
            auto: false,
            docs_path: None,
        }
    }
}

impl ApplyLayer for WorkspaceConfig {
    type Layer = WorkspaceLayer;
    fn apply_layer(&mut self, layer: Self::Layer) {
        let WorkspaceLayer {
            name,
            members,
            preferred_members,
            generate_index,
            auto,
            docs_path,
        } = layer;
        self.name.apply_opt(name);
        self.generate_index.apply_val(generate_index);
        self.members.apply_val(members);
        self.preferred_members.apply_val(preferred_members);
        self.auto.apply_val(auto);
        self.docs_path = docs_path
    }
}
