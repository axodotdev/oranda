use camino::Utf8Path;

/// root dir of oranda so we can set the tests/snapshots/ dir reliably
const ROOT_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn snapshot_settings() -> insta::Settings {
    let mut settings = insta::Settings::clone_current();
    let snapshot_dir = Utf8Path::new(ROOT_DIR).join("tests").join("snapshots");
    settings.set_snapshot_path(snapshot_dir);
    settings.set_prepend_module_to_snapshot(false);
    settings
}
