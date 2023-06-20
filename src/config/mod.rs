use std::collections::HashMap;

use camino::Utf8PathBuf;

pub mod oranda_config;
pub mod project;

use crate::errors::*;
pub use oranda_config::{
    AnalyticsConfig, ArtifactsConfig, BoolOr, FundingConfig, MdBookConfig, OrandaConfig,
    SocialConfig, StyleConfig,
};
use project::ProjectConfig;

#[derive(Debug)]
pub struct Config {
    pub description: String,
    pub dist_dir: String,
    pub homepage: Option<String>,
    pub static_dir: String,
    pub name: String,
    pub no_header: bool,
    pub readme_path: String,
    pub repository: Option<String>,
    pub analytics: Option<AnalyticsConfig>,
    pub additional_pages: Option<HashMap<String, String>>,
    pub social: Option<SocialConfig>,
    pub artifacts: ArtifactsConfig,
    pub version: Option<String>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub path_prefix: Option<String>,
    pub license: Option<String>,
    /// The config for using mdbook
    pub mdbook: Option<MdBookConfig>,
    pub styles: StyleConfig,
    pub changelog: bool,
    pub funding: Option<FundingConfig>,
}

impl Config {
    pub fn build(config_path: &Utf8PathBuf) -> Result<Config> {
        // Users can have multiple types of configuration or no configuration at all
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
        // We apply these in layers, with later layers winning over earlier ones.
        //
        // Note that several of these config merges do a seemingly-useless `if`
        // before applying a value. This is intentional to make the code more robust to refactors.
        //
        // If new stages are added or better defaults get introduced, we always
        // want to defer to those values if the layer we're currently applying doesn't have
        // an opinion on that value, which is what "None" in a config is really expressing.
        let mut cfg = Config::default();
        let custom = OrandaConfig::load(config_path)?;
        let project = ProjectConfig::load(None)?;

        cfg.apply_project_layer(project);
        cfg.apply_custom_layer(custom);
        cfg.find_mdbook();
        FundingConfig::find_paths(&mut cfg.funding)?;

        Ok(cfg)
    }

    /// Apply the layer of config we computed from project files
    fn apply_project_layer(&mut self, project: Option<ProjectConfig>) {
        if let Some(project) = project {
            self.name = project.name;
            self.description = project.description;
            self.homepage.apply_opt(project.homepage);
            self.repository.apply_opt(project.repository);
            self.version.apply_opt(project.version);
            self.license.apply_opt(project.license);
            self.readme_path
                .apply_val(project.readme_path.map(|p| p.to_string()));
            self.artifacts.cargo_dist.apply_opt(project.cargo_dist);
        }
    }

    /// Apply the layer of config we computed from oranda.json
    fn apply_custom_layer(&mut self, custom: Option<OrandaConfig>) {
        // Apply the "custom" layer
        if let Some(custom) = custom {
            self.description.apply_val(custom.description);
            self.dist_dir.apply_val(custom.dist_dir);
            self.static_dir.apply_val(custom.static_dir);
            self.homepage.apply_opt(custom.homepage);
            self.name.apply_val(custom.name);
            self.readme_path.apply_val(custom.readme_path);
            self.repository.apply_opt(custom.repository);
            self.analytics.apply_layer(custom.analytics);
            // FIXME: should this get merged with e.g. `extend?`
            self.additional_pages.apply_opt(custom.additional_pages);
            self.social.apply_layer(custom.social);
            self.artifacts.apply_val_layer(custom.artifacts);
            self.styles.apply_val_layer(custom.styles);
            self.logo.apply_opt(custom.logo);
            self.favicon.apply_opt(custom.favicon);
            self.path_prefix.apply_opt(custom.path_prefix);
            self.changelog.apply_val(custom.changelog);
            self.mdbook.apply_bool_layer(custom.mdbook);
            self.funding.apply_bool_layer(custom.funding);
        }
    }

    /// If mdbook is enabled but the path isn't set, we try to find it
    ///
    /// If we fail, we set mdbook to None to disable it.
    fn find_mdbook(&mut self) {
        if let Some(mdbook_cfg) = &mut self.mdbook {
            if mdbook_cfg.path.is_none() {
                // Ok time to auto-detect, try these dirs for a book.toml
                let possible_paths = vec!["./", "./book/", "./docs/"];
                for book_dir in possible_paths {
                    let book_path = Utf8PathBuf::from(book_dir).join("book.toml");
                    if book_path.exists() {
                        // nice, use it
                        mdbook_cfg.path = Some(book_dir.to_owned());
                        return;
                    }
                }
                // We found nothing, disable mdbook
                self.mdbook = None;
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            description: String::new(),
            dist_dir: String::from("public"),
            homepage: None,
            name: String::from("My Axo project"),
            no_header: false,
            readme_path: String::from("README.md"),
            repository: None,
            analytics: None,
            additional_pages: None,
            social: None,
            artifacts: ArtifactsConfig::default(),
            styles: StyleConfig::default(),
            version: None,
            license: None,
            logo: None,
            favicon: None,
            path_prefix: None,
            static_dir: String::from("static"),
            // Later stages can disable mdbook support by setting this to None
            mdbook: Some(MdBookConfig::default()),
            changelog: false,
            funding: Some(FundingConfig::default()),
        }
    }
}

// Utils for merging things

/// Trait for merging a new layer of config
pub trait ApplyLayer
where
    Self: Sized,
{
    /// Merges this value with another layer of itself, preferring the new layer
    fn apply_layer(&mut self, layer: Self);

    /// Merges this value with another layer of itself, preferring the new layer
    ///
    /// (asymteric case where the rhs is an Option but we're just A Value)
    fn apply_val_layer(&mut self, layer: Option<Self>) {
        if let Some(val) = layer {
            self.apply_layer(val);
        }
    }
}

/// Blanket impl of merging layers wrapped in Options
impl<T> ApplyLayer for Option<T>
where
    T: ApplyLayer,
{
    fn apply_layer(&mut self, layer: Self) {
        if let Some(val) = layer {
            if let Some(this) = self {
                this.apply_layer(val);
            } else {
                *self = Some(val);
            }
        }
    }
}

/// Extension trait to provide apply_bool_layer
pub trait ApplyBoolLayerExt {
    type Inner;
    /// Merge an `Option<Layer>` with an `Option<BoolOr<Layer>>`
    ///
    /// There are 3 cases for the rhs:
    ///
    /// * Some(Val): override; recursively apply_layer
    /// * Some(false): manually disabled; set lhs to None
    /// * Some(true) / None: redundant; do nothing
    fn apply_bool_layer(&mut self, layer: Option<BoolOr<Self::Inner>>);
}

impl<T> ApplyBoolLayerExt for Option<T>
where
    T: ApplyLayer,
{
    type Inner = T;
    fn apply_bool_layer(&mut self, layer: Option<BoolOr<Self::Inner>>) {
        match layer {
            Some(BoolOr::Val(val)) => {
                self.apply_layer(Some(val));
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
