use crate::errors::*;

use crate::config::Config;

fn fetch_additional_css(config: &Config) -> Result<String> {
    let mut css = vec![];
    for url in &config.additional_css {
        let additional_css_future = axoasset::copy(url, &config.dist_dir);

        let additional_path = tokio::runtime::Handle::current().block_on(additional_css_future)?;

        css.append(additional_path)
    }

    Ok(css)
}

pub fn build(config: &Config) -> Result<String> {
    let mut css = String::from("");
    if !config.additional_css.is_empty() {
        let additional_css = fetch_additional_css(config)?;

        css = format!("{css}{additional}", css = css, additional = additional_css);
    }
    Ok(css)
}
