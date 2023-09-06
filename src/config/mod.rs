//! This is the config subsystem!
//!
//! # Concepts
//!
//! It's responsible for loading, merging, and auto-detecting all the various config
//! sources. There are two closely related families of types:
//!
//! - `...Config` types are the "complete" values that will be passed around to the rest
//!   of the program. All of these types get shoved into the top-level [`Config`][] type.
//!
//! - `...Layer` types are "partial" values that are loaded and parsed before being merged
//!   into the final [`Config`][]. Notably the oranda.json is loaded as [`OrandaLayer`][] and
//!   Cargo.toml/package.json gets loaded as [`AxoprojectLayer`][].
//!
//! Nested types like [`ComponentConfig`][] usually have a paired layer ([`ComponentLayer`][]),
//! with an almost identical definition. The differences usually lie in the Layer having far more
//! Options, because you don't need to specify it in your oranda.json but we want the rest of our
//! code to have the final result fully resolved.
//!
//! The Big Idea is that:
//!
//! - a `...Config` type implements [`Default`][] manually to specify default values
//! - a `...Config` type implements [`ApplyLayer`][] to specify how its `...Layer` gets combined
//!
//! Conveniences like [`ApplyValExt::apply_val`][] and [`ApplyOptExt::apply_opt`][]
//! exist to help merge simple values like `bool <- Option<bool>` where overwriting the entire
//! value is acceptable.
//!
//! [`ApplyBoolLayerExt::apply_bool_layer`][] exists to apply [`BoolOr`][] wrappers
//! which lets oranda.json say things like `mdbook = false` when [`MdBookConfig`][]
//! is actually an entire struct.
//!
//!
//! # Top-Level Layers
//!
//! These are the current top-level """layers""" that get constructed and merged into
//! the top-level [`Config`][]. They are merged more free-form, but try to quickly shell
//! out to [`ApplyLayer`][] for consistency/reliability.
//!
//! The top-level layers are applied in the following order, with the later ones winning:
//!
//! - **The Default Layer** comes from [`Config::default`][] and the recursive [`Default`][]
//!   impls on the other `...Config` structs.
//!
//! - **[`AxoprojectLayer`][]** comes from a project manifest file. We currently
//!   support `Cargo.toml` and `package.json`, but could support any manifest
//!   that provides information like `name`, `description`, `repository`...
//!
//! - **[`OrandaLayer`][]**, AKA "the custom layer", comes from an `oranda.json` file.
//!   It's basically a complete replica of [`Config`][] but with way more Options.
//!
//! - **The Autodetect Layer** is just a convention where configs have an opportunity
//!   to try to find missing values, erroring out if they fail while the user
//!   was clearly trying to enable the feature.
//!
//! Note that several of these config merges are seemingly pedantic about preserving/merging
//! old values when only one source sets it in practice. This is to make the code more reliable,
//! consistent, and robust in the face of future config/layer additions without you having to
//! know exactly all the ways a value can be set.
//!
//!
//! # Schemas
//!
//! We use [`schemars::JsonSchema`][] to auto-derive the Json Schema
//! for oranda.json ([`OrandaLayer`][]). Schemars is aware of most serde annotations,
//! so it largely requires no configuration to work.
//!
//! The schema pulls docs from the doc-comments on the `...Layer` types, so be sure to write
//! those as if they're being shown as an on-hover tool-tip in an editor.
//!
//! Note that, as is conventional for Json Schemas, the derived schema allows unknown fields
//! to exist everywhere, so while this schema can help tell you what values we understand, it
//! can't help the user notice they have an unknown/typo'd key. Maybe we should tighten this up,
//! because that seems to be a super common issue, especially when we change the config.

// We very intentionally manually implement Default a lot in this submodule
// to keep things very explicit and clear
#![allow(clippy::derivable_impls)]

use std::convert::identity;

use camino::Utf8PathBuf;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::errors::*;

pub mod axoproject;
mod builds;
mod components;
mod marketing;
pub mod oranda_config;
pub mod project;
pub mod style;
mod workspace;

pub use self::axoproject::AxoprojectLayer;
pub use self::oranda_config::OrandaLayer;
pub use builds::{BuildConfig, BuildLayer};
pub use components::{
    ArtifactsConfig, ArtifactsLayer, ComponentConfig, ComponentLayer, FundingConfig, FundingLayer,
    MdBookConfig, MdBookLayer, PackageManagersConfig, PackageManagersLayer,
};
pub use marketing::{AnalyticsConfig, MarketingConfig, MarketingLayer, SocialConfig, SocialLayer};
pub use workspace::{WorkspaceConfig, WorkspaceLayer, WorkspaceMember};

pub use project::{ProjectConfig, ProjectLayer};
pub use style::{StyleConfig, StyleLayer};

/// Top-level mega-config
#[derive(Debug, Clone)]
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
    /// Workspace configuration
    pub workspace: WorkspaceConfig,
}

impl Config {
    pub fn build(config_path: &Utf8PathBuf) -> Result<Config> {
        // Load Layers
        let custom = OrandaLayer::load(config_path)?;
        let project = AxoprojectLayer::load(None)?;

        // default layer
        let mut cfg = Config::default();
        // axoproject layer
        cfg.apply_project_layer(project);
        // oranda.json layer
        cfg.apply_custom_layer(custom);
        // auto-detect layer
        cfg.apply_autodetect_layer(None)?;

        Ok(cfg)
    }

    /// Build out a config for the workspace root, which is only interested in what's in the
    /// oranda_workspace.json.
    pub fn build_workspace_root(config_path: &Utf8PathBuf) -> Result<Config> {
        // This loads the `oranda_workspace.json`
        let conf = OrandaLayer::load(config_path)?;

        // Does the loaded value exist and have a defined workspace member list?
        let (set_members, set_auto) = conf
            .as_ref()
            // Is there a conf?
            .and_then(|c| {
                // Is there a workspace?
                c.workspace.as_ref().map(|w| {
                    // Is there a members field?
                    let set_members = w.members.is_some();
                    // Is there an auto field, and is it set true?
                    let set_auto = w.auto.is_some_and(identity);
                    (set_members, set_auto)
                })
            })
            .unwrap_or((false, false));

        let mut cfg = Config::default();
        cfg.apply_custom_layer(conf);

        // If no members were set, attempt to set the member list from the
        // detected list
        if !set_members && set_auto {
            // This will never be `None`, since we already appended a path to it before.
            let root_path = config_path.parent().unwrap();
            let workspace = AxoprojectLayer::load_workspace(root_path)?;

            if let Some(detected_members) = workspace.and_then(|w| w.members) {
                cfg.workspace.members = detected_members;
            }
        }

        Ok(cfg)
    }

    /// Build out a config for a workspace member, which is interested in the following things,
    /// in ascending priority:
    /// - axoproject stuff
    /// - "root" config keys that are inherited
    /// - its own oranda.json
    /// - autodetect
    #[instrument("workspace_page", fields(prefix = prefix))]
    pub fn build_workspace_member(
        config_path: &Utf8PathBuf,
        root_config_path: &Utf8PathBuf,
        project_root: &Utf8PathBuf,
        workspace_member: &WorkspaceMember,
        prefix: Option<String>,
    ) -> Result<Config> {
        let member_conf = OrandaLayer::load(config_path)?;
        let root_conf = OrandaLayer::load(root_config_path)?;
        let project = AxoprojectLayer::load(Some(project_root.into()))?;

        // Complain if the member config contains workspace keys, because those keys should be set
        // in the top-level workspace config file.
        if let Some(member_conf) = &member_conf {
            if member_conf.workspace.is_some() {
                tracing::warn!("oranda.json contains workspace configuration! You likely want to set this in the root oranda-workspace.json.");
            }
        }

        let mut cfg = Config::default();
        cfg.apply_project_layer(project);
        cfg.apply_custom_layer(root_conf);
        cfg.apply_custom_layer(member_conf);
        cfg.apply_autodetect_layer(Some(workspace_member))?;

        Ok(cfg)
    }

    /// Apply the layer of config we computed from project files
    fn apply_project_layer(&mut self, layer: Option<AxoprojectLayer>) {
        if let Some(layer) = layer {
            // This is intentionally written slightly cumbersome to make you update this
            let AxoprojectLayer {
                project,
                cargo_dist,
                members: _,
            } = layer;

            self.project.apply_val_layer(project);
            if let Some(artifacts) = &mut self.components.artifacts {
                artifacts.cargo_dist.apply_val(cargo_dist);
            }
        }
    }

    /// Apply the layer of config we computed from oranda.json
    fn apply_custom_layer(&mut self, layer: Option<OrandaLayer>) {
        if let Some(layer) = layer {
            // This is intentionally written slightly cumbersome to make you update this
            let OrandaLayer {
                project,
                build,
                marketing,
                styles,
                components,
                workspace,
                _schema,
            } = layer;
            self.project.apply_val_layer(project);
            self.build.apply_val_layer(build);
            self.marketing.apply_val_layer(marketing);
            self.styles.apply_val_layer(styles);
            self.components.apply_val_layer(components);
            self.workspace.apply_val_layer(workspace);
        }
    }

    /// Apply the layer of config that does auto-detection of missing values
    fn apply_autodetect_layer(&mut self, workspace_member: Option<&WorkspaceMember>) -> Result<()> {
        // Find out if we need to start in another directory, in case we're working under a
        // workspace. If not, we start in the current directory.
        let start_dir = workspace_member
            .map(|m| m.path.clone())
            .unwrap_or(".".into());
        MdBookConfig::find_paths(&mut self.components.mdbook, &start_dir)?;
        FundingConfig::find_paths(&mut self.components.funding, &start_dir)?;

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
            workspace: WorkspaceConfig::default(),
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
    T: ApplyLayer + Default,
{
    type Inner = T::Layer;

    /// Apply a layer that can either be a boolean, or a Layer Value (most likely an object).
    ///
    /// Possible cases (lhs is the resultant config, rhs is the incoming layer):
    /// lhs == Some && rhs == true  = nothing happens
    /// lhs == Some && rhs == false = lhs gets set to None
    /// lhs == Some && rhs == value = layer gets applied to lhs
    /// lhs == None && rhs == true  = lhs gets set to layer default
    /// lhs == None && rhs == false = nothing happens
    /// lhs == None && rhs == value = lhs gets set to layer default with layer applied
    /// rhs = nothing               = we do nothing
    fn apply_bool_layer(&mut self, layer: Option<BoolOr<Self::Inner>>) {
        match layer {
            Some(BoolOr::Val(val)) => {
                if let Some(this) = self {
                    this.apply_layer(val);
                } else {
                    let mut t = T::default();
                    t.apply_layer(val);
                    *self = Some(t);
                }
            }
            Some(BoolOr::Bool(false)) => {
                // Disable this setting
                *self = None;
            }
            Some(BoolOr::Bool(true)) => {
                // Enable if self was previously set to None
                if self.is_none() {
                    *self = Some(T::default());
                }
            }
            None => {}
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
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(untagged)]
pub enum BoolOr<T> {
    /// They gave the simple bool
    Bool(bool),
    /// They gave a more interesting value
    Val(T),
}
