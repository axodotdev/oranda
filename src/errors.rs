use thiserror::Error;

pub type Result<T> = std::result::Result<T, OrandaError>;

#[derive(Debug, Error)]
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
    AxoAsset(#[from] axoasset::AxoassetError),

    #[error(transparent)]
    FSExtra(#[from] fs_extra::error::Error),

    #[error("failed to read {filedesc} at {path}")]
    FileNotFound { filedesc: String, path: String },

    #[error("Could not find a build in {dist_dir}. Did you remember to run `oranda build`?")]
    BuildNotFound { dist_dir: String },

    #[error("{0}")]
    Other(String),
}
