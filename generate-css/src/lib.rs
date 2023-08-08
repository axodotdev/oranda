pub mod errors;

extern crate axoasset;
extern crate camino;
extern crate directories;
extern crate miette;
extern crate thiserror;

use crate::errors::Result;
use axoasset::LocalAsset;
use camino::{Utf8Path, Utf8PathBuf};
use directories::ProjectDirs;
use std::env;
use std::io::Write;
use std::process::Command;

const MANIFEST_PATH: &str = env!("CARGO_MANIFEST_DIR");
const CSS_SRC_PATH: &str = "../oranda-css/css/main.css";
const TAILWIND_SRC_PATH: &str = "../oranda-css/tailwind.config.js";
const DEFAULT_CSS_OUTPUT_DIR: &str = "../oranda-css/dist";

fn manifest_dir() -> &'static Utf8Path {
    Utf8Path::new(MANIFEST_PATH)
}

pub fn default_css_output_dir() -> Utf8PathBuf {
    manifest_dir().join(DEFAULT_CSS_OUTPUT_DIR)
}

pub fn build_css(dist_dir: &Utf8Path) -> Result<()> {
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
        #[cfg(target_family = "unix")]
        {
            use std::os::unix::prelude::PermissionsExt;
            let user_execute = std::fs::Permissions::from_mode(0o755);
            std::fs::set_permissions(&binary_path, user_execute)?;
        }
    }

    tracing::info!("Building oranda CSS using Tailwind...");
    let css_src_path = manifest_dir().join(CSS_SRC_PATH);
    let tailwind_config_path = manifest_dir().join(TAILWIND_SRC_PATH);
    let output_path = dist_dir.join("oranda.css");
    let output = Command::new(binary_path)
        .args([
            "-c",
            tailwind_config_path.as_str(),
            "-i",
            css_src_path.as_str(),
            "-o",
            output_path.as_str(),
            "--minify",
        ])
        .output()?;
    std::io::stderr().write_all(&output.stderr)?;
    output
        .status
        .success()
        .then_some(true)
        .expect("Tailwind failed to compile CSS!");

    Ok(())
}
