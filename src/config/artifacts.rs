use axohtml::elements::div;
use axohtml::{html, text};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use std::path::PathBuf;

use crate::errors::*;

use super::Config;

/// The final report of cargo-dist
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DistManifest {
    /// App releases we're distributing
    pub releases: Vec<Release>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Release {
    /// The name of the app
    pub app_name: String,
    /// The version of the app
    // FIXME: should be a Version but JsonSchema doesn't support (yet?)
    pub app_version: String,
    /// The artifacts for this release (zips, debuginfo, metadata...)
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Artifact {
    /// The unique name of the artifact (e.g. `myapp-v1.0.0-x86_64-pc-windows-msvc.zip`)
    pub name: String,
    /// The kind of artifact this is (e.g. "exectuable-zip")
    #[serde(flatten)]
    pub kind: ArtifactKind,
    /// The target triple of the bundle
    pub target_triple: Option<String>,
    /// The location of the artifact on the local system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    /// Assets included in the bundle (like executables)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<Asset>>,
    /// A string describing how to install this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_hint: Option<String>,
}

/// An asset contained in an artifact (executable, license, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Asset {
    /// The high-level name of the asset
    pub name: String,
    /// The path of the asset relative to the root of the artifact
    pub path: PathBuf,
    /// The kind of asset this is
    #[serde(flatten)]
    pub kind: AssetKind,
}

/// An artifact included in a Distributable
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum AssetKind {
    /// An executable artifact
    #[serde(rename = "executable")]
    Executable(ExecutableAsset),
    /// A README file
    #[serde(rename = "readme")]
    Readme,
    /// A LICENSE file
    #[serde(rename = "license")]
    License,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind")]
pub enum ArtifactKind {
    /// A zip or a tarball
    #[serde(rename = "executable-zip")]
    ExecutableZip,
    /// Standalone Symbols/Debuginfo for a build
    #[serde(rename = "symbols")]
    Symbols,
    /// Machine-readable metadata
    #[serde(rename = "dist-metadata")]
    DistMetadata,
    /// Installer
    #[serde(rename = "installer")]
    Installer,
}

/// An executable artifact (exe/binary)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExecutableAsset {
    /// The name of the Artifact containing symbols for this executable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols_artifact: Option<String>,
}

impl DistManifest {}

#[derive(Debug, Deserialize)]
pub struct Artifacts {
    pub cargo_dist: bool,
}

pub fn create_artifacts_tabs(config: &Config) -> Result<Option<Box<div<String>>>> {
    let Some(Artifacts { cargo_dist: true }) = &config.artifacts else {
        return Ok(None);
      };

    if config.repository.is_none() || config.version.is_none() {
        return Err(OrandaError::Other(String::from(
            "The repository and version are required for cargo_dist",
        )));
    }

    let url = format!(
        "{}/releases/download/v{}/dist-manifest.json",
        config.repository.as_ref().unwrap(),
        config.version.as_ref().unwrap()
    );

    let resp = reqwest::blocking::get(url);

    let Ok(resp) = resp else {
        return Err(OrandaError::Other(String::from(
            "The repository and version configurations are required for cargo_dist",
        )));
      };

    let typed = &resp.json::<DistManifest>().unwrap();

    println!("{:?}", typed.releases);
    let mut html: Vec<Box<div<String>>> = vec![];
    for release in typed.releases.iter() {
        for artifact in release.artifacts.iter() {
            html.extend(html!(<div>{text!(&artifact.name)}</div>));
        }
    }

    return Ok(Some(
        html!(<div><h2 class="text-center">{text!("Download for your platform")}</h2>{html}</div>),
    ));
}
