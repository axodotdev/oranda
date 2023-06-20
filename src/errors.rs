use axoasset::AxoassetError;
use camino::Utf8PathBuf;
use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, OrandaError>;

#[derive(Debug, Diagnostic, Error)]
pub enum OrandaError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),

    #[error(transparent)]
    StripPrefixError(#[from] std::path::StripPrefixError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Syntect(#[from] syntect::Error),

    #[error(transparent)]
    #[diagnostic(transparent)]
    AxoAsset(#[from] axoasset::AxoassetError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Octolotl(#[from] octolotl::OctolotlError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("Failed to create a directory, `{dist_path}` to build your project in.")]
    DistDirCreationError {
        dist_path: String,
        #[source]
        details: std::io::Error,
    },

    #[error("Found an invalid value, `{path}`, assigned to ORANDA_CSS environment variable.")]
    #[diagnostic(help("Please make sure you give a valid path pointing to a css file."))]
    InvalidOrandaCSSOverride { path: String },

    #[error("Failed fetching releases from Github.")]
    GithubReleasesFetchError {
        #[source]
        details: octolotl::OctolotlError,
    },

    #[error("Failed parsing response when fetching releases from Github.")]
    GithubReleaseParseError {
        #[source]
        details: axoasset::AxoassetError,
    },

    #[error("Could not find any releases from {repo_owner}/{repo_name} with a cargo-dist compatible `dist-manifest.json`.")]
    NoCargoDistReleasesFound {
        repo_owner: String,
        repo_name: String,
    },

    #[error(transparent)]
    FSExtra(#[from] fs_extra::error::Error),

    #[error("failed to read {filedesc} at {path}")]
    FileNotFound { filedesc: String, path: String },

    #[error("failed to parse your repo, current config has repo as: {repo}")]
    #[diagnostic(help("please make sure this is correct."))]
    RepoParseError {
        repo: String,
        #[diagnostic_source]
        details: miette::Report,
    },

    #[error("Could not find a build in {dist_dir}")]
    #[diagnostic(help("Did you remember to run `oranda build`?"))]
    BuildNotFound { dist_dir: String },

    #[error("Skipping malformed dist-manifest.json for {tag}")]
    #[diagnostic(severity = "warn")]
    CargoDistManifestMalformed {
        tag: String,
        #[diagnostic_source]
        details: AxoassetError,
    },

    #[error("Skipping unparseable dist-manifest.json for {tag}")]
    #[diagnostic(help(
        "the schema was version {schema_version}, while our parser is version {parser_version}"
    ))]
    #[diagnostic(severity = "warn")]
    CargoDistManifestPartial {
        schema_version: String,
        parser_version: String,
        tag: String,
        #[diagnostic_source]
        details: AxoassetError,
    },

    #[error("Couldn't load your mdbook at {path}")]
    MdBookLoad {
        path: String,
        #[source]
        details: mdbook::errors::Error,
    },

    #[error("Couldn't build your mdbook at {path}")]
    MdBookBuild {
        path: String,
        #[source]
        details: mdbook::errors::Error,
    },
    #[error("We found a potential {kind} project at {manifest_path} but there was an issue")]
    #[diagnostic(severity = "warn")]
    BrokenProject {
        kind: String,
        manifest_path: Utf8PathBuf,
        #[diagnostic_source]
        cause: axoproject::errors::AxoprojectError,
    },
    #[error("Failed to loading funding details at {path}")]
    #[diagnostic(severity = "warn")]
    FundingLoadFailed {
        path: Utf8PathBuf,
        #[diagnostic_source]
        details: axoasset::AxoassetError,
    },
    /// This error indicates we tried to deserialize some TOML with toml_edit
    /// but failed.
    #[error("Failed to edit toml document")]
    TomlEdit {
        /// The SourceFile we were trying to parse
        #[source_code]
        source: axoasset::SourceFile,
        /// The range the error was found on
        #[label]
        span: Option<miette::SourceSpan>,
        /// Details of the error
        #[source]
        details: toml_edit::TomlError,
    },

    #[error("We were unable to watch your filesystem for changes")]
    #[diagnostic(help = "Make sure that oranda has privileges to set up file watchers!")]
    FilesystemWatchError(#[from] notify_debouncer_mini::notify::Error),

    #[error("Failed to fetch your funding info from GitHub.")]
    #[diagnostic(help = "Make sure that your funding file is located at `.github/FUNDING.yml`.")]
    GithubFundingFetchError {
        #[source]
        details: reqwest::Error,
    },

    #[error("Couldn't find your FUNDING.yml or funding.md")]
    #[diagnostic(help = "You can manually specify md_path or yml_path in your funding config")]
    FundingConfigInvalid,

    #[error("Error while parsing FUNDING.yml")]
    #[diagnostic(
        help = "Make sure your FUNDING.yml conforms to GitHub's format!",
        url = "https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/displaying-a-sponsor-button-in-your-repository"
    )]
    GithubFundingParseError { details: String },

    #[error("{0}")]
    Other(String),
}
