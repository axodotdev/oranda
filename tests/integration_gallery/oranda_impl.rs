use std::sync::Mutex;

use crate::utils::snapshots::snapshot_settings;
use camino::{Utf8Path, Utf8PathBuf};
use miette::IntoDiagnostic;
use oranda::config::{OrandaLayer, WorkspaceLayer, WorkspaceMember};

use super::command::CommandInfo;
use super::errors::Result;
use super::repo::{Repo, TestContext, TestContextLock, ToolsImpl};

/// Set this to change the date that we clamp Github Releases data to
/// (prevents new Github Releases from breaking our tests, can be increased
/// manually to update )
///
/// (RFC 3339 entry on utctime.net)
const DEFAULT_DATA_CLAMP: &str = "2023-08-08T20:56:30+00:00";
/// Set this at runtime to override DEFAULT_DATA_CLAMP
const ENV_DATA_CLAMP: &str = "DEBUG_DATA_CLAMP_DATE";
/// Set this at runtime to override STATIC_CARGO_DIST_BIN
const ENV_RUNTIME_ORANDA_BIN: &str = "OVERRIDE_CARGO_BIN_EXE_oranda";
/// oranda binary that was built with `cargo test`
const STATIC_ORANDA_BIN: &str = env!("CARGO_BIN_EXE_oranda");
static TOOLS: Mutex<Option<Tools>> = Mutex::new(None);

/// axolotlsay 0.1.0 is a nice simple project with shell+powershell+npm installers in its release
pub static AXOLOTLSAY: TestContextLock<Tools> = TestContextLock::new(
    &TOOLS,
    &Repo {
        repo_owner: "oranda-gallery",
        repo_name: "axolotlsay",
        commit_ref: "main",
        app_name: "axolotlsay",
        subdir: None,
        bins: &["axolotlsay"],
    },
);
/// akaikatana-repack 0.2.0 has multiple bins!
pub static AKAIKATANA_REPACK: TestContextLock<Tools> = TestContextLock::new(
    &TOOLS,
    &Repo {
        repo_owner: "oranda-gallery",
        repo_name: "akaikatana-repack",
        commit_ref: "main",
        app_name: "akaikatana-repack",
        subdir: None,
        bins: &["akextract", "akmetadata", "akrepack"],
    },
);

/// a repo with cargo-dist, but no releases
pub static EMPTY_TEST: TestContextLock<Tools> = TestContextLock::new(
    &TOOLS,
    &Repo {
        repo_owner: "oranda-gallery",
        repo_name: "oranda-empty-test",
        commit_ref: "main",
        app_name: "oranda-empty-test",
        subdir: None,
        bins: &["oranda-empty-test"],
    },
);

/// a repo to test artifact inference
pub static INFERENCE_TEST: TestContextLock<Tools> = TestContextLock::new(
    &TOOLS,
    &Repo {
        repo_owner: "oranda-gallery",
        repo_name: "oranda-inference-test",
        commit_ref: "main",
        app_name: "oranda-inference-test",
        subdir: None,
        bins: &["oranda-inference-test"],
    },
);

/// it's oranda!
pub static ORANDA: TestContextLock<Tools> = TestContextLock::new(
    &TOOLS,
    &Repo {
        repo_owner: "oranda-gallery",
        repo_name: "oranda",
        commit_ref: "main",
        app_name: "oranda",
        subdir: None,
        bins: &["oranda"],
    },
);

pub struct Tools {
    pub git: CommandInfo,
    pub oranda: CommandInfo,
    pub temp_root: Utf8PathBuf,
}

impl Tools {
    fn new() -> Self {
        eprintln!("getting tools...");
        let git = CommandInfo::new("git", None).expect("git isn't installed");

        // If OVERRIDE_* is set, prefer that over the version that cargo built for us,
        // this lets us test our shippable builds.
        let oranda_path =
            std::env::var(ENV_RUNTIME_ORANDA_BIN).unwrap_or_else(|_| STATIC_ORANDA_BIN.to_owned());
        let oranda = CommandInfo::new("oranda", Some(&oranda_path)).expect("oranda isn't built!?");

        // If a data clamp isn't set, set one ourselves
        if std::env::var(ENV_DATA_CLAMP).is_err() {
            std::env::set_var(ENV_DATA_CLAMP, DEFAULT_DATA_CLAMP);
        }

        // Setup the tempdir / oranda-workspace.json
        const TARGET_TEMP_DIR: &str = env!("CARGO_TARGET_TMPDIR");
        let this = Self {
            git,
            oranda,
            temp_root: Utf8PathBuf::from(TARGET_TEMP_DIR),
        };
        this.init_temp_workspace_root();

        this
    }

    fn init_temp_workspace_root(&self) {
        let temp_root = self.temp_root();
        let public = self.oranda_workspace_dist();
        if public.exists() {
            std::fs::remove_dir_all(public).expect("failed to delete temp workspace 'public' dir");
        }
        if !temp_root.exists() {
            std::fs::create_dir_all(temp_root).expect("failed to create temp workspace dir");
        }

        let json = OrandaLayer {
            project: None,
            build: None,
            marketing: None,
            styles: None,
            components: None,
            workspace: Some(WorkspaceLayer {
                name: Some("oranda gallery".to_owned()),
                generate_index: Some(true),
                members: Some(vec![]),
                preferred_members: None,
                auto: Some(false),
                docs_path: None,
            }),
            _schema: None,
        };
        self.save_oranda_workspace_json(&json)
            .expect("failed to create oranda-workspace.json");
    }

    pub fn temp_root(&self) -> &Utf8Path {
        &self.temp_root
    }

    pub fn oranda_workspace_json(&self) -> Utf8PathBuf {
        self.temp_root().join("oranda-workspace.json")
    }

    pub fn oranda_workspace_dist(&self) -> Utf8PathBuf {
        self.temp_root().join("public")
    }

    pub fn save_oranda_workspace_json(&self, val: &OrandaLayer) -> Result<()> {
        let json_src = serde_json::to_string_pretty(val).into_diagnostic()?;
        axoasset::LocalAsset::write_new(&json_src, self.oranda_workspace_json())?;
        Ok(())
    }

    pub fn load_oranda_workspace_json(&self) -> Result<OrandaLayer> {
        let json_src = axoasset::SourceFile::load_local(self.oranda_workspace_json())?;
        let json = json_src.deserialize_json()?;
        Ok(json)
    }
}

impl ToolsImpl for Tools {
    fn git(&self) -> &CommandInfo {
        &self.git
    }
}
impl Default for Tools {
    fn default() -> Self {
        Self::new()
    }
}

pub struct OrandaResult {
    test_name: String,
    public_dir: Option<Utf8PathBuf>,
}

impl<'a> TestContext<'a, Tools> {
    /// Run `oranda build` with the JSON configuration that we provided which sets up our fake
    /// workspace
    pub fn oranda_build(&self, test_name: &str) -> Result<OrandaResult> {
        eprintln!("running oranda build...");
        self.tools.oranda.output_checked(|cmd| cmd.arg("build"))?;

        self.load_oranda_results(test_name)
    }

    fn load_oranda_results(&self, test_name: &str) -> Result<OrandaResult> {
        // read/analyze installers
        eprintln!("loading results...");

        // Patch ourselves into things properly
        if test_name != "gal_workspace" {
            let mut json = self.tools.load_oranda_workspace_json()?;
            json.workspace
                .as_mut()
                .unwrap()
                .members
                .as_mut()
                .unwrap()
                .push(WorkspaceMember {
                    slug: test_name.to_owned(),
                    path: self.working_dir.as_std_path().to_owned(),
                });
            self.tools.save_oranda_workspace_json(&json)?;
        }

        let public_dir = self.working_dir.join("public");

        Ok(OrandaResult {
            test_name: test_name.to_owned(),
            public_dir: public_dir.exists().then_some(public_dir),
        })
    }

    pub fn load_oranda_json(&self) -> Result<oranda::config::OrandaLayer> {
        eprintln!("loading oranda.json...");
        let json_src = axoasset::SourceFile::load_local("oranda.json")?;
        let json = json_src.deserialize_json()?;
        Ok(json)
    }
    pub fn save_oranda_json(&self, json: oranda::config::OrandaLayer) -> Result<()> {
        eprintln!("storing oranda.json...");
        let json_src = serde_json::to_string_pretty(&json).into_diagnostic()?;
        axoasset::LocalAsset::write_new(&json_src, "oranda.json")?;
        Ok(())
    }
}

impl OrandaResult {
    pub fn check_all(&self) -> Result<()> {
        // Now that all other checks have passed, it's safe to check snapshots
        self.snapshot()?;

        Ok(())
    }

    // Run cargo-insta on everything we care to snapshot
    pub fn snapshot(&self) -> Result<()> {
        eprintln!("snapshotting...");
        // We make a single uber-snapshot to avoid the annoyances of having multiple snapshots in one test
        let mut snapshots = String::new();

        let Some(src_path) = &self.public_dir else {
            return Ok(());
        };
        for path in glob::glob(&format!("{}/**/*.html", src_path)).unwrap() {
            let path = path.unwrap();

            let path = Utf8PathBuf::from_path_buf(path).unwrap();
            // We don't want to test another tool's output, so we filter out mdbook files. This
            // is kind of a FIXME, the way we do this is very brittle, we should really be disabling
            // mdbook from running altogether.
            if !&path.to_string().contains("book") {
                let rel_path = pathdiff::diff_utf8_paths(&path, src_path).unwrap();
                // Normalize Windows slashes to Unix slashes
                let rel_path = rel_path.to_string().replace('\\', "/");
                Self::append_snapshot_file(
                    &mut snapshots,
                    &format!("public/{}", rel_path),
                    Some(path.as_path()),
                )
                .unwrap();
            }
        }

        let test_name = &self.test_name;
        snapshot_settings_with_oranda_css_filter().bind(|| {
            insta::assert_snapshot!(format!("{test_name}-public"), &snapshots);
        });
        Ok(())
    }

    fn append_snapshot_file(
        out: &mut String,
        name: &str,
        src_path: Option<&Utf8Path>,
    ) -> Result<()> {
        // `glob` guarantees this path exists
        let src_path = src_path.unwrap();
        let src = axoasset::LocalAsset::load_string(src_path)?;
        Self::append_snapshot_string(out, name, &src)
    }

    fn append_snapshot_string(out: &mut String, name: &str, val: &str) -> Result<()> {
        use std::fmt::Write;

        writeln!(out, "================ {name} ================").unwrap();
        writeln!(out, "{val}").unwrap();
        Ok(())
    }
}

pub fn snapshot_settings_with_version_filter() -> insta::Settings {
    let mut settings = snapshot_settings();
    settings.add_filter(
        r"\d+\.\d+\.\d+(\-prerelease\d*)?(\.\d+)?",
        "1.0.0-FAKEVERSION",
    );
    settings
}

pub fn snapshot_settings_with_oranda_css_filter() -> insta::Settings {
    let mut settings = snapshot_settings();
    settings.add_filter(
        r"oranda(-v\d+\.\d+\.\d+(\-prerelease\d*)?(\.\d+)?)?.css",
        "oranda.css",
    );
    settings
}

pub fn snapshot_settings_with_dist_manifest_filter() -> insta::Settings {
    let mut settings = snapshot_settings_with_version_filter();
    settings.add_filter(
        r#""announcement_tag": .*"#,
        r#""announcement_tag": "CENSORED","#,
    );
    settings.add_filter(
        r#""announcement_title": .*"#,
        r#""announcement_title": "CENSORED""#,
    );
    settings.add_filter(
        r#""announcement_changelog": .*"#,
        r#""announcement_changelog": "CENSORED""#,
    );
    settings.add_filter(
        r#""announcement_github_body": .*"#,
        r#""announcement_github_body": "CENSORED""#,
    );
    settings.add_filter(
        r#""announcement_is_prerelease": .*"#,
        r#""announcement_is_prerelease": "CENSORED""#,
    );
    settings.add_filter(
        r#""cargo_version_line": .*"#,
        r#""cargo_version_line": "CENSORED""#,
    );
    settings
}
