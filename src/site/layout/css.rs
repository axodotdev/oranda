use crate::config::style::ORANDA_CSS_TAG;
use std::env;
use std::io::Write;
use std::process::Command;
use std::sync::RwLock;

use crate::errors::*;

use axoasset::{Asset, LocalAsset};
use camino::{Utf8Path, Utf8PathBuf};
use directories::ProjectDirs;
use minifier::css;

static CSS_CACHE: RwLock<Vec<CssItem>> = RwLock::new(Vec::new());

const CSS_SRC_PATH: &str = "oranda-css/css/main.css";

struct CssItem {
    release_tag: String,
    contents: String,
}

fn concat_minify(css_files: &[String]) -> Result<String> {
    let mut css = String::new();
    for file in css_files {
        let future = Asset::load_string(file);
        let unminified = tokio::runtime::Handle::current().block_on(future)?;
        let minified = match css::minify(&unminified) {
            Ok(css) => Ok(css),
            Err(e) => Err(OrandaError::Other(e.to_string())),
        };
        css = format!("{css}/* {file} */{minified}", minified = minified?);
    }

    Ok(css)
}

pub fn get_css_link(path_prefix: &Option<String>, release_tag: &str) -> Result<String> {
    let filename = get_css_filename(release_tag);
    Ok(crate::site::link::generate(path_prefix, &filename))
}

/// Places the CSS into the output folder. Depending on if we're running in local development or in
/// a released binary, the method of obtaining said CSS will differ.
pub fn place_css(dist_dir: &str, release_tag: &str) -> Result<()> {
    // Even if you're running a development build, we still respect the custom CSS version preference
    // by falling back to fetching said version from GitHub.
    if cfg!(debug_assertions) && release_tag == ORANDA_CSS_TAG {
        // If we're running in a local environment, we fetch a Tailwind binary and compile the CSS
        // on the spot. This is useful if we're working on oranda-css locally.
        build_css(dist_dir)
    } else {
        // If we're running in a released binary, we fetch the latest tag (or whichever tag is
        // specified in the configuration).
        fetch_css(dist_dir, release_tag)
    }
}

pub fn build_css(dist_dir: &str) -> Result<()> {
    // Fetch our cache dir
    let project_dir = ProjectDirs::from("dev", "axo", "oranda")
        .expect("Unable to create cache dir for downloading Tailwind!");
    let cache_dir = project_dir.cache_dir();
    // Figure out our target "double" (tailwind has weird naming around this)
    let double = match (env::consts::OS, env::consts::ARCH) {
        ("linux", "x86_64") => "linux-x64",
        ("linux", "aarch64") => "linux-arm64",
        ("linux", "arm") => "linux-armv7",
        ("macos", "x86_64") => "macos-x64",
        ("macos", "aarch64") => "macos-arm64",
        ("windows", "x86_64") => "windows-x64.exe",
        ("windows", "aarch64") => "windows-arm64.exe",
        _ => "linux-x64",
    };
    let mut binary_path = Utf8PathBuf::from(cache_dir.display().to_string());
    LocalAsset::create_dir_all(&binary_path)?;
    binary_path.push(format!("tailwindcss-{double}"));
    if !binary_path.exists() {
        // Fetch the binary from GitHub if it doesn't exist
        tracing::info!("Fetching Tailwind binary from GitHub release...");
        let url = format!(
            "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-{double}"
        );
        let handle = tokio::runtime::Handle::current();
        let response = handle.block_on(reqwest::get(url))?;
        let bytes = handle.block_on(response.bytes())?;
        let file = LocalAsset::new(&binary_path, Vec::from(bytes))?;
        file.write(
            binary_path
                .parent()
                .expect("Tailwind binary path has no parent!?"),
        )?;

        // On non-Windows platforms, we need to mark the file as executable
        if !double.starts_with("windows") {
            Command::new("chmod")
                .args(["+x", binary_path.as_str()])
                .output()?;
        }
    }

    tracing::info!("Building oranda CSS using Tailwind...");
    let css_src_path = Utf8PathBuf::from(CSS_SRC_PATH);
    let output = Command::new(binary_path)
        .args([
            "-c",
            "oranda-css/tailwind.config.js",
            "-i",
            css_src_path.as_str(),
            "-o",
            &format!("{dist_dir}/oranda.css"),
            "--minify",
        ])
        .output()?;
    std::io::stderr().write_all(&output.stderr)?;

    Ok(())
}

fn fetch_css(dist_dir: &str, release_tag: &str) -> Result<()> {
    match env::var("ORANDA_CSS") {
        Ok(path) => {
            let msg = format!("Overriding oranda_css path with {}", &path);
            tracing::warn!("{}", &msg);
            LocalAsset::copy(&path, dist_dir)?;
            Ok(())
        }
        Err(_) => {
            let filename = format!("oranda-{release_tag}.css");
            let dest_path = Utf8Path::new(dist_dir).join(filename);

            // Do we already have this value cached?
            let cache_val = {
                let cache = CSS_CACHE.read().expect("CSS Cache should not be poisoned");
                cache
                    .iter()
                    .find(|elem| elem.release_tag.as_str() == release_tag)
                    .map(|elem| elem.contents.clone())
            };

            let oranda_css_response = if let Some(c) = cache_val {
                // Yes, we do!
                c
            } else {
                // Nope, sure don't. Get it, and if we are successful, store it for next time.
                let fresh =
                    tokio::runtime::Handle::current().block_on(fetch_oranda(release_tag))?;

                let mut cache = CSS_CACHE.write().expect("CSS Cache should not be poisoned");
                cache.push(CssItem {
                    release_tag: release_tag.to_string(),
                    contents: fresh.clone(),
                });
                fresh
            };

            LocalAsset::write_new_all(&oranda_css_response, dest_path)?;
            Ok(())
        }
    }
}

async fn fetch_oranda(release_tag: &str) -> Result<String> {
    let oranda_css_request =
        octolotl::request::ReleaseAsset::new("axodotdev", "oranda", release_tag, "oranda.css");
    Ok(octolotl::Request::send(&oranda_css_request, true)
        .await?
        .text()
        .await?)
}

fn get_css_filename(release_tag: &str) -> String {
    if (cfg!(debug_assertions) && release_tag == ORANDA_CSS_TAG) || env::var("ORANDA_CSS").is_ok() {
        "oranda.css".into()
    } else {
        format!("oranda-{release_tag}.css")
    }
}

pub fn write_additional_css(additional_css: &[String], dist_dir: &Utf8Path) -> Result<()> {
    let minified_css = concat_minify(additional_css)?;

    LocalAsset::write_new(&minified_css, dist_dir.join("custom.css"))?;
    Ok(())
}
