use crate::config::Config;
use crate::errors::*;
use crate::site::asset;

pub fn build(config: &Config) -> Result<String> {
    let css_options = grass::Options::default().style(grass::OutputStyle::Compressed);
    let mut css = grass::from_path("src/site/css/stylesheets/style.scss", &css_options)?;
    Ok(css)
}
