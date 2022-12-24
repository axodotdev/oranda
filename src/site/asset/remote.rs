use std::fmt;
use std::path::{Path, PathBuf};

use crate::site::asset::error::*;

#[derive(Debug)]
pub struct RemoteAsset {
    pub origin_path: String,
    pub label: String,
    pub response: reqwest::blocking::Response,
}

impl fmt::Display for RemoteAsset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} asset at path, {}, ", self.label, self.origin_path)
    }
}

impl RemoteAsset {
    pub fn load(origin_path: &str, label: &str) -> Result<RemoteAsset> {
        match reqwest::blocking::get(origin_path) {
            Ok(response) => Ok(RemoteAsset {
                origin_path: origin_path.to_string(),
                label: label.to_string(),
                response,
            }),
            Err(details) => Err(AxoassetError::RemoteAssetRequestFailed {
                origin_path: origin_path.to_string(),
                label: label.to_string(),
                details: details.to_string(),
            }),
        }
    }

    pub fn copy(origin_path: &str, label: &str, dist_dir: &str) -> Result<PathBuf> {
        match RemoteAsset::load(origin_path, label) {
            Ok(mut a) => {
                let dist_path = a.dist_path(dist_dir)?;
                let mut file = std::fs::File::create(&dist_path)?;
                match a.response.copy_to(&mut file) {
                    Ok(_) => Ok(dist_path),
                    Err(details) => Err(AxoassetError::RemoteAssetCopyFailed {
                        asset: a,
                        dist_path: dist_path.display().to_string(),
                        details: details.to_string(),
                    }),
                }
            }
            Err(details) => Err(AxoassetError::RemoteAssetLoadFailed {
                origin_path: origin_path.to_string(),
                label: label.to_string(),
                details: details.to_string(),
            }),
        }
    }

    fn dist_path(&self, dist_dir: &str) -> Result<PathBuf> {
        let filename = self.filename()?;
        Ok(Path::new(&dist_dir).join(filename))
    }

    fn mimetype(&self) -> Result<mime::Mime> {
        let headers = self.response.headers();
        match headers.get(reqwest::header::CONTENT_TYPE) {
            Some(content_type) => {
                let mtype: mime::Mime = content_type.to_str()?.parse()?;
                match mtype.type_() {
                    mime::IMAGE => Ok(mtype),
                    _ => Err(AxoassetError::RemoteAssetNonImageMimeType {
                        asset: self.to_string(),
                    }),
                }
            }
            None => Err(AxoassetError::RemoteAssetMissingContentTypeHeader {
                asset: self.to_string(),
            }),
        }
    }

    fn extension(&self) -> Result<String> {
        let mimetype = self.mimetype()?;
        if let Some(img_format) = image::ImageFormat::from_mime_type(&mimetype) {
            let extensions = img_format.extensions_str();
            if !extensions.is_empty() {
                Ok(extensions[0].to_string())
            } else {
                Err(
                    AxoassetError::RemoteAssetIndeterminateImageFormatExtension {
                        asset: self.to_string(),
                    },
                )
            }
        } else {
            Err(AxoassetError::RemoteAssetMimeTypeNotSupported {
                asset: self.to_string(),
                mimetype: mimetype.to_string(),
            })
        }
    }

    fn filename(&self) -> Result<String> {
        Ok(format!("logo.{}", self.extension()?))
    }
}
