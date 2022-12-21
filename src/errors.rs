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

    #[error("failed to fetch {asset} at {url}: Encountered an error when requesting a remote asset. Make sure the url you prodived is accurate. Details:\r{details}")]
    RemoteAssetRequestFailed {
        asset: String,
        url: String,
        details: String,
    },

    #[error("failed to fetch {asset} at {origin_path}: Could not find asset at provided path. Make sure your path is relative to your oranda config or project manifest file. Details:\r{details} ")]
    LocalAssetNotFound {
        asset: String,
        origin_path: String,
        details: String,
    },

    #[error("failed to copy {asset} from {origin_path} to {dist_path}: Could not find asset at provided path. Make sure your path is relative to your oranda config or project manifest file. Details:\r{details}")]
    LocalAssetCopyFailed {
        asset: String,
        origin_path: String,
        dist_path: String,
        details: String,
    },

    #[error("{asset} url scheme, {origin_path}, did not match http or https: Please use an http or https url or a local path.")]
    RemoteAssetPathSchemeNotSupported { asset: String, origin_path: String },

    #[error("could not parse {asset} url, {origin_path}: Please use an http or https url or a local path. Details:\r{details}")]
    RemoteAssetPathParseError {
        asset: String,
        origin_path: String,
        details: String,
    },

    #[error("{0}")]
    Other(String),
}
