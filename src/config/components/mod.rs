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
#[derive(Debug, Clone)]
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
/// Extra components
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ComponentLayer {
    /// Whether to enable the changelog page
    ///
    /// In the future this may become more complex, but for now this is just a bool
    pub changelog: Option<bool>,
    /// The config for building and embedding an mdbook on your site
    ///
    /// The book will be linked as "docs" in the nav, and restyled to match
    /// the theme you have for the rest of your oranda site. We use a vendored
    /// copy of mdbook, and not the one you have installed on your system.
    ///
    /// This feature is enabled by default if we find a "book.toml" in
    /// "./", "./book/", or "./docs/".
    ///
    /// It can be completely disabled by setting `"mdbook": false`.
    ///  
    /// More precise settings can be used with `"mdbook": { ... }`.
    pub mdbook: Option<BoolOr<MdBookLayer>>,
    /// The config for for the "funding" page
    ///
    /// This feature is enabled by default if we find a file at "funding.md"
    /// or "./.github/FUNDING.yml".
    ///
    /// It can be completely disabled by setting `"funding": false`.
    ///  
    /// More precise settings can be used with `"funding": { ... }`.
    pub funding: Option<BoolOr<FundingLayer>>,
    /// The config for the "install" page and widget
    ///
    /// # Data Sources
    ///
    /// Once enabled we consider data for 3 possible data sources:
    ///
    /// * The `components.artifacts.package_managers` keys (see those configs for details)
    /// * Your GitHub Releases (if we have a URL to a GitHub repo in `project.repository`)
    /// * dist-manifest.json in your GitHub Releases (if cargo-dist support is enabled)
    ///
    /// These sources can give us:
    ///
    /// * Downloadable Files (`my-app-x86_64-pc-windows-msvc.zip`)
    /// * Runnable One-Liners (`curl install.sh | sh`, `npm install ...`)
    ///
    /// Which can be known/assumed to be:
    ///
    /// * Runnable/Installable on specific platforms (don't recommend `install.sh` on windows)
    /// * Preferred over others (install script nicer than downloadable tarballs)
    /// * Have checksums (`my-app-x86_64-pc-windows-msvc.zip.sha256`)
    /// * Be viewable (`install.sh`)
    ///
    /// GitHub Releses support will also get us a list of Releases these will be grouped into
    /// We will try to intelligently pick a "Latest Release" to display up-front:
    ///
    /// * If there's cargo-dist releases, prefer the latest one of those (stable over prereleases)
    ///     * This is a hack to make monorepos behave better while we don't fully support them
    /// * Otherwise if there's a stable release, pick the latest stable release
    /// * Otherwise pick the latest prerelease
    ///
    /// Otherwise we will place all of this under one artifical "current release".
    ///
    ///
    /// ## Github Releases
    ///
    /// GitHub Releases integration is a fuzzy system of auto-detection based on the filenames
    /// of assets we find uploaded to each GitHub Release. If we find a filename contains
    /// something that looks like a target triple ("x86_64-pc-windows-msvc") we will assume
    /// the artifact is only for that platform.
    ///
    /// If we find files with names like "..install..sh" we will assume those are curl-sh
    /// installer scripts.
    ///
    /// If we find a platform-specific tarball/zip, we will assume it's an archive containing
    /// prebuilt binaries for that platform.
    ///
    /// We also experimentally detect things like .deb files but it's only half-baked right now.
    ///
    ///
    /// # The Install Page
    ///
    /// The Install Page will show up in your nav if this feature is at all enabled.
    ///
    /// It will show:
    ///
    /// 1. The Install Widget (see below)
    /// 2. All the Runnable One-Liners for the Latest Release
    /// 3. A table of downloadable artifacts (tarballs/zips) for the Latest Release.
    ///
    ///
    /// # The Install Widget
    ///
    /// The Install Widget will show up on your front page and install page if
    /// this feature is at all enabled. Although currently it will auto-hide
    /// if there's absolutely nothing to show (#305).
    ///
    /// It will autodetect the user's current platform and try to recommend an installation method.
    ///
    /// If there are multiple supported platforms with different installation methods,
    /// a "platform" dropdown will appear to override the auto-detect.
    ///
    /// If there are multiple installation options for a platform, we will present them as tabs,
    /// roughly in decreasing order of preference (first tab is always best).
    ///
    /// Preference is currently fuzzy, but roughly:
    ///
    /// * platform-native solutions like msi's or debs will be recommended first
    /// * installer scripts will be shown after that
    /// * custom package_managers will be shown after that
    /// * tarballs/zips will be shown last
    ///
    /// If no installation methods are found for the user's platform, they will be linked to
    /// the install page and offered the platform dropdown to override.
    ///
    ///
    /// # Enabling / Disabling
    ///
    /// This feature is enabled if:
    ///
    /// * cargo-dist support is enabled
    ///   * `artifacts.cargo_dist: true` is set
    ///   * we find `[workspace.metadata.dist]` in your Cargo.toml
    /// * you set a package_manager in `artifacts.package_managers`
    ///
    /// It can be completely disabled by setting `"artifacts": false`.
    ///  
    /// More precise settings can be used with `"artifacts": { ... }`.
    ///
    /// FIXME(#397): there is currently an expressivity hole here: there is no way to *just*
    /// turn on plain GitHub Releases integration. You either need to have cargo-dist
    /// integration enabled, or add a random package_manager to make us enable it.
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
impl ComponentConfig {
    /// Convenience for checking if the artifacts component is actually enabled
    /// because a ton of code was repeating this due to the extra Option.
    pub fn artifacts_enabled(&self) -> bool {
        self.artifacts
            .as_ref()
            .map(|a| a.has_some())
            .unwrap_or(false)
    }
}
