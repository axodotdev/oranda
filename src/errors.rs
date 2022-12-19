use thiserror::Error;

pub type Result<T> = std::result::Result<T, OrandaError>;

#[derive(Debug, Error)]
pub enum OrandaError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Syntect(#[from] syntect::Error),

    #[error(transparent)]
    Grass(#[from] Box<grass::Error>),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("failed to read {filedesc} at {path}")]
    FileNotFound { filedesc: String, path: String },

    #[error("failed to fetch {resource} at {url}")]
    RequestFailed { resource: String, url: String },

    #[error("{0}")]
    Other(String),
}
