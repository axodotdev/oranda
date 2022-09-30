use super::options::Options;
use crate::Site;
use std::{fs::File, io::Write};

pub fn create_site_files(parsed_options: Options, site: Site) -> Result<(), std::io::Error> {
    let dist = &parsed_options.dist.unwrap_or_default();
    std::fs::create_dir_all(&dist)?;
    let html_path = format!("{}/index.html", &dist);
    let css_path = format!("{}/styles.css", &dist);

    let mut html_file = File::create(html_path)?;
    html_file.write_all(site.html.as_bytes())?;

    let mut css_file = File::create(css_path)?;
    css_file.write_all(site.css.as_bytes())?;

    Ok(())
}
