use assert_fs::prelude::*;
use assert_fs::TempDir;
use camino::Utf8PathBuf;

use oranda::config::Config;

pub fn from_json(json: serde_json::Value, dir: &mut TempDir) -> Config {
    let c = dir.child("oranda.json");
    c.write_str(&json.to_string())
        .expect("Unable to write oranda.json");
    let mut config = Config::build(&Utf8PathBuf::from_path_buf(c.path().to_path_buf()).unwrap())
        .expect("Unable to generate config");
    config.build.dist_dir = dir.path().display().to_string();
    // Override repository, except if it's non-standard
    if config
        .project
        .repository
        .as_ref()
        .is_some_and(|repo| repo == "https://github.com/axodotdev/oranda")
    {
        config.project.repository = Some("https://github.com/oranda-gallery/oranda".to_string());
    }
    config
}
