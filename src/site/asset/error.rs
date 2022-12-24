use thiserror::Error;

use crate::site::asset::local::LocalAsset;
use crate::site::asset::remote::RemoteAsset;

pub type Result<T> = std::result::Result<T, AxoassetError>;

#[derive(Debug, Error)]
pub enum AxoassetError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    ReqwestHeaderParse(#[from] reqwest::header::ToStrError),

    #[error(transparent)]
    MimeParseParse(#[from] mime::FromStrError),

    #[error("failed to fetch {label} at {origin_path}: Encountered an error when requesting a remote asset. Make sure the url you prodived is accurate. Details:\r{details}")]
    RemoteAssetRequestFailed {
        origin_path: String,
        label: String,
        details: String,
    },

    #[error("failed to fetch {label} at {origin_path}: Encountered an error when requesting a remote asset. Make sure the url you prodived is accurate. Details:\r{details}")]
    RemoteAssetLoadFailed {
        origin_path: String,
        label: String,
        details: String,
    },

    #[error("{label} url, {origin_path}, did not use http or https: Please use an http or https url or a local path.")]
    RemoteAssetPathSchemeNotSupported { origin_path: String, label: String },

    #[error("when fetching {asset}, the server's response mime type did not indicate an image: Please make sure the asset url is correct and that the server is properly configured")]
    RemoteAssetNonImageMimeType { asset: String },

    #[error("failed to copy {0} from {1} to {dist_path}: Encountered an error copying server response body to filesystem. Make sure your server is configured correctly and your destination path has the correct permissions. Details:\r{details}", asset.label, asset.origin_path)]
    RemoteAssetCopyFailed {
        asset: RemoteAsset,
        dist_path: String,
        details: String,
    },

    #[error("when fetching {asset}, the server responded with a mime type that was non supported: Please make sure the asset url is correct and that the server is properly configured")]
    RemoteAssetMimeTypeNotSupported { asset: String, mimetype: String },

    #[error("when fetching {asset}, we could not determine an appropriate file extension based on the server response: Please make sure the asset url is correct and that the server is properly configured")]
    RemoteAssetIndeterminateImageFormatExtension { asset: String },

    #[error("when fetching {asset}, the server's response did not contain a content type header: Please make sure the asset url is correct and that the server is properly configured")]
    RemoteAssetMissingContentTypeHeader { asset: String },

    #[error("could not parse {label} url, {origin_path}: Please use an http or https url or a local path. Details:\r{details}")]
    RemoteAssetPathParseError {
        origin_path: String,
        label: String,
        details: String,
    },

    #[error("failed to fetch {label} at {origin_path}: Could not find asset at provided path. Make sure your path is relative to your oranda config or project manifest file. Details:\r{details}")]
    LocalAssetNotFound {
        origin_path: String,
        label: String,
        details: String,
    },

    #[error("failed to copy {0} from {1} to {dist_path}: Could not find asset at provided path. Make sure your path is relative to your oranda config or project manifest file. Details:\r{details}", asset.label, asset.origin_path)]
    LocalAssetCopyFailed {
        asset: LocalAsset,
        dist_path: String,
        details: String,
    },

    #[error("failed to copy {asset} to {dist_path}: Could not find asset at provided path. Make sure your path is relative to your oranda config or project manifest file. Details:\r{details}")]
    LocalAssetWriteFailed {
        asset: String,
        dist_path: String,
        details: String,
    },

    #[error("could not determine file name for {asset}: Make sure your path is relative to your oranda config or project manifest file.")]
    LocalAssetMissingFilename { asset: String },
}
