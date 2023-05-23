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
        details: reqwest::Error,
    },

    #[error("Failed parsing response when fetching releases from Github.")]
    GithubReleaseParseError {
        #[source]
        details: reqwest::Error,
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

    #[error("Encountered an error (status: {status_code}) while fetching your cargo-dist release manifest at {url}.")]
    #[diagnostic(help("This often occurs when you haven't yet published a GitHub release for the version set in your Cargo.toml. Consider publishing a release or promoting a draft release for the current version, or updating your Cargo.toml to use an already released version."))]
    CargoDistManifestFetchError {
        url: String,
        status_code: reqwest::StatusCode,
    },

    #[error("Encountered an error parsing your cargo-dist manifest at {url}.")]
    CargoDistManifestParseError {
        url: String,
        #[source]
        details: reqwest::Error,
    },

    #[error("Couldn't load your mdbook at {path}")]
    MdBookLoad {
        path: String,
        inner: mdbook::errors::Error,
    },

    #[error("Couldn't build your mdbook at {path}")]
    MdBookBuild {
        path: String,
        inner: mdbook::errors::Error,
    },
    #[error("We found a potential {kind} project at {manifest_path} but there was an issue")]
    #[diagnostic(severity = "warn")]
    BrokenProject {
        kind: String,
        manifest_path: Utf8PathBuf,
        #[diagnostic_source]
        cause: axoproject::errors::AxoprojectError,
    },

    #[error("We were unable to watch your filesystem for changes")]
    #[diagnostic(help = "Make sure that oranda has privileges to set up file watchers!")]
    FilesystemWatchError(#[from] notify::Error),

    #[error("{0}")]
    Other(String),
}
