use schemars::JsonSchema;
use serde::Deserialize;

mod artifacts;
mod funding;
mod mdbooks;

pub use artifacts::{ArtifactsConfig, ArtifactsLayer, PackageManagersConfig, PackageManagersLayer};
pub use funding::{FundingConfig, FundingLayer};
pub use mdbooks::{MdBookConfig, MdBookLayer};

use super::{ApplyBoolLayerExt, ApplyLayer, ApplyValExt, BoolOr};

/// Extra components (complete version)
#[derive(Debug)]
pub struct ComponentConfig {
    /// Whether to enable the changelog page
    ///
    /// In the future this may become more complex, but for now this is it
    pub changelog: bool,
    /// The config for using mdbook for a "docs" page
    ///
    /// This defaults to Some(Default) and is set to None
    /// if we fail to auto-detect necessary information or if the user
    /// manually disables it.
    pub mdbook: Option<MdBookConfig>,
    /// The config for for the funding page
    ///
    /// This defaults to Some(Default) and is set to None
    /// if we fail to auto-detect necessary information or if the user
    /// manually disables it.
    pub funding: Option<FundingConfig>,
    /// The config for the "install" page and widget
    ///
    /// This defaults to Some(Default) and is set to None
    /// if we fail to auto-detect necessary information or if the user
    /// manually disables it.
    pub artifacts: Option<ArtifactsConfig>,
}
/// Extra components (partial version used by oranda.json)
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ComponentLayer {
    /// Whether to enable the changelog page
    ///
    /// In the future this may become more complex, but for now this is it
    pub changelog: Option<bool>,
    /// The config for using mdbook for a "docs" page
    ///
    /// This defaults to Some(Default) and is set to None
    /// if we fail to auto-detect necessary information or if the user
    /// manually disables it.
    pub mdbook: Option<BoolOr<MdBookLayer>>,
    /// The config for for the funding page
    ///
    /// This defaults to Some(Default) and is set to None
    /// if we fail to auto-detect necessary information or if the user
    /// manually disables it.
    pub funding: Option<BoolOr<FundingLayer>>,
    /// The config for the "install" page and widget
    ///
    /// This defaults to Some(Default) and is set to None
    /// if we fail to auto-detect necessary information or if the user
    /// manually disables it.
    pub artifacts: Option<BoolOr<ArtifactsLayer>>,
}
impl Default for ComponentConfig {
    fn default() -> Self {
        ComponentConfig {
            changelog: false,
            mdbook: Some(MdBookConfig::default()),
            funding: Some(FundingConfig::default()),
            artifacts: Some(ArtifactsConfig::default()),
        }
    }
}
impl ApplyLayer for ComponentConfig {
    type Layer = ComponentLayer;
    fn apply_layer(&mut self, layer: Self::Layer) {
        // This is intentionally written slightly cumbersome to make you update this
        let ComponentLayer {
            changelog,
            mdbook,
            funding,
            artifacts,
        } = layer;
        self.changelog.apply_val(changelog);
        self.mdbook.apply_bool_layer(mdbook);
        self.funding.apply_bool_layer(funding);
        self.artifacts.apply_bool_layer(artifacts);
    }
}
