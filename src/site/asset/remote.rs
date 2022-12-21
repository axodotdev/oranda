use std::path::{Path, PathBuf};

use crate::errors::*;

pub fn copy(dist_dir: &str, origin_path: &str, label: &str) -> Result<PathBuf> {
    let img = reqwest::blocking::get(origin_path)?;
    let headers = img.headers();
    let dist_path = dist_path(dist_dir, origin_path, headers, label)?;
    let mut file = std::fs::File::create(dist_path)?;
    img.copy_to(&mut file)?;
    Ok(dist_path)
}

fn mimetype(
    origin_path: &str,
    headers: &reqwest::header::HeaderMap,
    label: &str,
) -> Result<mime::Mime> {
    match headers.get(reqwest::header::CONTENT_TYPE) {
        Some(content_type) => {
            let mtype: mime::Mime = content_type.to_str()?.parse()?;
            match mtype.type_() {
                mime::IMAGE => Ok(mtype),
                _ => Err(OrandaError::RemoteAssetNonImageMimeType {
                    asset: label.to_string(),
                    origin_path: origin_path.to_string(),
                }),
            }
        }
        None => Err(OrandaError::RemoteAssetMissingContentTypeHeader {
            asset: label.to_string(),
            origin_path: origin_path.to_string(),
        }),
    }
}

fn extension(mimetype: &mime::Mime, label: &str) -> Result<String> {
    if let Some(img_format) = image::ImageFormat::from_mime_type("image/png") {
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
    origin_path: &str,
    headers: &reqwest::header::HeaderMap,
    label: &str,
) -> Result<String> {
    let mimetype = mimetype(origin_path, headers, label)?;
    Ok(format!("logo.{}", extension(&mimetype, label)?))
}

fn dist_path(
    dist_dir: &str,
    origin_path: &str,
    headers: &reqwest::header::HeaderMap,
    label: &str,
) -> Result<PathBuf> {
    let filename = filename(origin_path, headers, label)?;
    Ok(Path::new(&dist_dir).join(filename))
}
