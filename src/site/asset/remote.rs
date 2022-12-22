use std::path::{Path, PathBuf};

use crate::errors::*;

pub struct RemoteAsset {
    dist_dir: String,
    origin_path: String,
    label: String,
}

impl RemoteAsset {
    pub fn copy(&self) -> Result<PathBuf> {
        let img = reqwest::blocking::get(self.origin_path)?;
        let headers = img.headers();
        let dist_path = self.dist_path(headers)?;
        let mut file = std::fs::File::create(dist_path)?;
        img.copy_to(&mut file)?;
        Ok(dist_path)
    }

    fn dist_path(&self, headers: &reqwest::header::HeaderMap) -> Result<PathBuf> {
        let filename = filename(self.origin_path, headers, self.label)?;
        Ok(Path::new(&self.dist_dir).join(filename))
    }

    fn mimetype(
        &self,
        headers: &reqwest::header::HeaderMap,
    ) -> Result<mime::Mime> {
        match headers.get(reqwest::header::CONTENT_TYPE) {
            Some(content_type) => {
                let mtype: mime::Mime = content_type.to_str()?.parse()?;
                match mtype.type_() {
                    mime::IMAGE => Ok(mtype),
                    _ => Err(OrandaError::RemoteAssetNonImageMimeType {
                        asset: self,
                    }),
                }
            }
            None => Err(OrandaError::RemoteAssetMissingContentTypeHeader {
                asset: self,
            }),
        }
    }

    fn extension(&self, headers: &reqwest::header::HeaderMap) -> Result<String> {
        let mimetype = self.mimetype(headers)?;
        if let Some(img_format) = image::ImageFormat::from_mime_type(mimetype) {
            let extensions = img_format.extensions_str();
            if !extensions.is_empty() {
                Ok(extensions[0].to_string())
            } else {
                Err(OrandaError::RemoteAssetIndeterminateImageFormatExtension {
                    asset: label.to_string(),
                })
            }
        } else {
            Err(OrandaError::RemoteAssetMimeTypeNotSupported {
                asset: label.to_string(),
                mimetype: mimetype.to_string(),
            })
        }
    }

    fn filename(
        &self,
        headers: &reqwest::header::HeaderMap,
    ) -> Result<String> {
        Ok(format!("logo.{}", self.extension(headers)?))
    }
}
