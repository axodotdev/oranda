use std::path::Path;

use crate::config::Config;
use axohtml::elements::img;
use axohtml::html;

use crate::errors::*;

pub fn get_logo(config: &Config) -> Option<Result<Box<img<String>>>> {
    config
        .logo
        .to_owned()
        .map(|logo_origin_path| fetch(&config.dist_dir, logo_origin_path))
}

fn fetch(dist_dir: &str, origin_path: String) -> Result<Box<img<String>>> {
    if Path::new(&origin_path).exists() {
        let new_path = match axoasset::copy(&origin_path, "Logo", &dist_dir) {
            Ok(path) => {
                let path_as_string = path.to_str().unwrap();
                println!("OMG HERE {:?}", path_as_string);
                return Ok(html!(<img src=path_as_string />));
            }
            Err(_) => Err(OrandaError::Other(
                "There was a problem copying your logo".to_owned(),
            )),
        };

        new_path
    } else {
        Err(OrandaError::FileNotFound {
            filedesc: "Logo".to_owned(),
            path: origin_path.to_string(),
        })
    }
}
