// We very intentionally manually implement Default a lot in this submodule
// to keep things very explicit and clear
#![allow(clippy::derivable_impls)]

use camino::Utf8PathBuf;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::errors::*;

pub mod axoproject;
mod builds;
mod components;
mod marketing;
pub mod oranda_config;
pub mod project;
pub mod style;

pub use self::axoproject::AxoprojectConfig;
pub use self::oranda_config::OrandaConfig;
pub use builds::{BuildConfig, BuildLayer};
pub use components::{
    ArtifactsConfig, ArtifactsLayer, ComponentConfig, ComponentLayer, FundingConfig, FundingLayer,
    MdBookConfig, MdBookLayer, PackageManagersConfig, PackageManagersLayer,
};
pub use marketing::{AnalyticsConfig, MarketingConfig, MarketingLayer, SocialConfig, SocialLayer};

pub use project::{ProjectConfig, ProjectLayer};
pub use style::{StyleConfig, StyleLayer};

/// Top-level mega-config
#[derive(Debug)]
pub struct Config {
    /// Info about the project/application
    pub project: ProjectConfig,
    /// Info about the build/output
    pub build: BuildConfig,
    /// Info about social/marketing/analytics
    pub marketing: MarketingConfig,
    /// Info about layout/themes
    pub styles: StyleConfig,
    /// Additional optional components
    pub components: ComponentConfig,
}

impl Config {
    pub fn build(config_path: &Utf8PathBuf) -> Result<Config> {
        // Users can have multiple types of configuration or no configuration at all
        //
        // - Default configuration comes from Config::default and the recursive Default
        //   impls on the other `*Config` structs.
        //
        // - Project configuration comes from a project manifest file. We currently
        //   support `Cargo.toml` and `package.json`, but could support any manifest
        //   that provided a `name`, `description`, `repository` and `homepage` field.
        //
        // - Custom configuration comes from a `oranda.config.json` file. If this
        //   file exists, it has precedence over project configuration, which means
        //   you could use this file to override fields in your project manifest.
        //   This file can contain all possible public configuration fields.
        //
        // - Auto-detect layer is just a convention where configs have an opportunity
        //   to try to find missing values, erroring out if they fail while the user
        //   was clearly trying to enable the feature.
        //
        // We apply these in layers, with later layers winning over earlier ones.
        //
        // Note that several of these config merges do a seemingly-useless `if`
        // before applying a value. This is intentional to make the code more robust to refactors.
        //
        // If new stages are added or better defaults get introduced, we always
        // want to defer to those values if the layer we're currently applying doesn't have
        // an opinion on that value, which is what "None" in a config is really expressing.
        let custom = OrandaConfig::load(config_path)?;
        let project = AxoprojectConfig::load(None)?;

        // default layer
        let mut cfg = Config::default();
        // axoproject layer
        cfg.apply_project_layer(project);
        // oranda.json layer
        cfg.apply_custom_layer(custom);
        // auto-detect layer
        cfg.apply_autodetect_layer()?;
        Ok(cfg)
    }

    /// Apply the layer of config we computed from project files
    fn apply_project_layer(&mut self, layer: Option<AxoprojectConfig>) {
        if let Some(layer) = layer {
            // This is intentionally written slightly cumbersome to make you update this
            let AxoprojectConfig {
                project,
                cargo_dist,
            } = layer;

            self.project.apply_layer(project);
            if let Some(artifacts) = &mut self.components.artifacts {
                artifacts.cargo_dist.apply_val(cargo_dist);
            }
        }
    }

    /// Apply the layer of config we computed from oranda.json
    fn apply_custom_layer(&mut self, layer: Option<OrandaConfig>) {
        if let Some(layer) = layer {
            // This is intentionally written slightly cumbersome to make you update this
            let OrandaConfig {
                project,
                build,
                marketing,
                styles,
                components,
            } = layer;
            self.project.apply_val_layer(project);
            self.build.apply_val_layer(build);
            self.marketing.apply_val_layer(marketing);
            self.styles.apply_val_layer(styles);
            self.components.apply_val_layer(components);
        }
    }

    /// Apply the layer of config that does auto-detection of missing values
    fn apply_autodetect_layer(&mut self) -> Result<()> {
        MdBookConfig::find_paths(&mut self.components.mdbook)?;
        FundingConfig::find_paths(&mut self.components.funding)?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            project: ProjectConfig::default(),
            build: BuildConfig::default(),
            marketing: MarketingConfig::default(),
            styles: StyleConfig::default(),
            components: ComponentConfig::default(),
        }
    }
}

// Utils for merging things

/// Trait for merging a new layer of config
pub trait ApplyLayer
where
    Self: Sized,
{
    /// The much more Option-ridden version of this config
    /// that can be repeatedly layerd with options
    type Layer;

    /// Merges this value with another layer of itself, preferring the new layer
    fn apply_layer(&mut self, layer: Self::Layer);

    /// Merges this value with another layer of itself, preferring the new layer
    ///
    /// (asymteric case where the rhs is an Option but we're just A Value)
    fn apply_val_layer(&mut self, layer: Option<Self::Layer>) {
        if let Some(val) = layer {
            self.apply_layer(val);
        }
    }
}

/// Extension trait to provide apply_bool_layer
pub trait ApplyBoolLayerExt {
    type Inner;
    /// Merge an `Option<Layer>` with an `Option<BoolOr<Layer>>`
    ///
    /// There are 3 cases for the rhs (layer):
    ///
    /// * Some(Val): override; recursively apply_layer
    /// * Some(false): manually disabled; set lhs to None
    /// * Some(true) / None: redundant; do nothing
    ///
    /// There are 2 cases for the lhs (self):
    ///
    /// * Some: still live, can be overriden/merged
    /// * None: permanently disabled, rhs will be ignored
    fn apply_bool_layer(&mut self, layer: Option<BoolOr<Self::Inner>>);
}

impl<T> ApplyBoolLayerExt for Option<T>
where
    T: ApplyLayer,
{
    type Inner = T::Layer;
    fn apply_bool_layer(&mut self, layer: Option<BoolOr<Self::Inner>>) {
        match layer {
            Some(BoolOr::Val(val)) => {
                if let Some(this) = self {
                    this.apply_layer(val);
                } else {
                    // If self is None, then
                }
            }
            Some(BoolOr::Bool(false)) => {
                // Disable this setting
                *self = None;
            }
            None | Some(BoolOr::Bool(true)) => {
                // Do nothing, use the previous value
                //
                // (Arguably "true" should mean something like Some(default)
                // but that's already the default and we don't want to clobber
                // other layers if they have an opinion.)
            }
        }
    }
}

/// Extension trait to provide apply_val
pub trait ApplyValExt
where
    Self: Sized,
{
    /// Merges a `T` with an `Option<T>`
    ///
    /// Overwrites the lhs if the rhs is Some
    fn apply_val(&mut self, layer: Option<Self>);
}
impl<T> ApplyValExt for T {
    fn apply_val(&mut self, layer: Option<Self>) {
        if let Some(val) = layer {
            *self = val;
        }
    }
}

/// Extension trait to provide apply_opt
pub trait ApplyOptExt
where
    Self: Sized,
{
    /// Merges an `Option<T>` with an `Option<T>`
    ///
    /// Overwrites the lhs if the rhs is Some
    fn apply_opt(&mut self, layer: Self);
}
impl<T> ApplyOptExt for Option<T> {
    fn apply_opt(&mut self, layer: Self) {
        if let Some(val) = layer {
            *self = Some(val);
        }
    }
}

/// A value or just a boolean
///
/// This allows us to have a simple yes/no version of a config while still
/// allowing for a more advanced version to exist.
#[derive(Deserialize, Debug, JsonSchema)]
#[serde(untagged)]
pub enum BoolOr<T> {
    /// They gave the simple bool
    Bool(bool),
    /// They gave a more interesting value
    Val(T),
}
