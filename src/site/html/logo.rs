use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{config::Config, errors::*};

pub fn get_logo(config: &Config) -> Result<Option<&PathBuf>> {
    let mut logo_url;

    if config.logo.is_empty() {
        return Ok(None);
    }

    if config.logo.starts_with("http") {
        let resp = reqwest::blocking::get(&config.logo);

        match resp {
            Err(_) => {
                return Err(OrandaError::RequestFailed {
                    url: config.logo.to_string(),
                    resource: String::from("Logo"),
                });
            }
            Ok(img) => {
                let logo_path = Path::join(
                    Path::new(&config.dist_dir),
                    Path::new(&config.logo).file_name().unwrap(),
                );

                let mut logo_file = File::create(&logo_path)?;
                logo_file.write_all(&img.bytes().unwrap())?;

                logo_url = &logo_path;
            }
        }
    } else {
        if !Path::new(&config.logo).exists() {
            return Err(OrandaError::FileNotFound {
                filedesc: "Logo".to_owned(),
                path: config.logo.to_owned(),
            });
        }

        let new_path = Path::join(
            Path::new(&config.dist_dir),
            Path::new(&config.logo).file_name().unwrap(),
        );
        fs::copy(&config.logo, &new_path).unwrap();
        logo_url = &new_path;
    }

    Ok(Some(&logo_url))
}
