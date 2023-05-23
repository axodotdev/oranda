mod artifacts;

use crate::config::{analytics, Config};
use crate::errors::*;
use crate::site::link;

use axoasset::LocalAsset;
use axohtml::{elements::script, html};
use camino::Utf8Path;

pub struct Analytics {
    pub snippet: Option<Box<script<String>>>,
    pub google_script: Option<Box<script<String>>>,
}

impl Analytics {
    pub fn new(config: &Config) -> Result<Self> {
        let snippet = analytics::snippet(config);
        match &config.analytics {
            Some(analytics::Analytics::Google(g)) => {
                let google_script = Some(g.get_script());
                Ok(Self {
                    snippet,
                    google_script,
                })
            }
            Some(_) => Ok(Self {
                snippet,
                google_script: None,
            }),
            None => Ok(Self {
                snippet: None,
                google_script: None,
            }),
        }
    }
}

pub fn build_os_script(path_prefix: &Option<String>) -> String {
    let script_url = link::generate(path_prefix, "artifacts.js");
    let script: Box<script<String>> = html!(<script src=script_url />);
    script.to_string()
}

pub fn write_os_script(dist_dir: &Utf8Path) -> Result<()> {
    LocalAsset::write_new(artifacts::SCRIPT, dist_dir.join("artifacts.js"))?;
    Ok(())
}
