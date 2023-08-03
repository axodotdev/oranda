use miette::Diagnostic;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, GenerateCssError>;

#[derive(Debug, Diagnostic, Error)]
pub enum GenerateCssError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    AxoAsset(#[from] axoasset::AxoassetError),
}
