use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
use grass::OutputStyle;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use utils::{create_site_files::create_site_files, options::Options};

use crate::utils::make_footer::make_footer;
use crate::utils::make_head::make_head;
use crate::utils::syntax_highlight::syntax_highlight;
use errors::*;

pub mod errors;
#[cfg(test)]
mod tests;
pub mod utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    // TODO: report useful paths/details for other tools
}

fn get_nested_toml(value: &Value, key: String) -> String {
    let empty_default = Value::String("".to_string());

    value["package"]
        .get(key)
        .unwrap_or(&empty_default)
        .to_string()
}

fn check_config_files() -> () {
    let mut name = String::new();
    let mut description = String::new();
    let mut homepage = String::new();

    let oranda_config_file = ".oranda.config.json";
    let cargo_file = "Cargo.toml";
    let package_json_file = "package.json";

    if Path::new(cargo_file).exists() {
        let file = fs::read_to_string(cargo_file).unwrap();

        let value: Value = toml::from_str(&file).unwrap();

        name = get_nested_toml(&value, "name".to_string());
        description = get_nested_toml(&value, "description".to_string());
        homepage = get_nested_toml(&value, "homepage".to_string());
    };

    if Path::new(package_json_file).exists() {
        let file = fs::read_to_string(package_json_file).unwrap();
        let value: serde_json::Value = serde_json::from_str(&file).unwrap();
        name = value["name"].to_string();
        description = value["description"].to_string();
        homepage = value["homepage"].to_string();
    };

    return;
}

pub fn do_oranda() -> Result<Report> {
    let parsed_options = Options::build();
    let file = parsed_options.file.as_ref();
    let mut file = File::open(file.unwrap())?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let site = create_site(&data, &parsed_options);
    match create_site_files(parsed_options, site) {
        Err(_) => Err(OrandaError::Other(
            "There was a problem creating your website files".to_owned(),
        )),
        Ok(_) => {
            let report = Report {};

            Ok(report)
        }
    }
}

fn initialize_comrak_options() -> ComrakOptions {
    let mut options = ComrakOptions::default();

    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.extension.description_lists = true;

    options
}

pub struct Site {
    pub html: String,
    pub css: String,
}

pub fn create_site(md: &str, options: &Options) -> Site {
    let comrak_options = initialize_comrak_options();
    let mut plugins = ComrakPlugins::default();

    pub struct MockAdapter {}
    impl SyntaxHighlighterAdapter for MockAdapter {
        fn highlight(&self, lang: Option<&str>, code: &str) -> String {
            let highlighted_code = syntax_highlight(lang, code);

            // requires a string to be returned
            match highlighted_code {
                Ok(code) => code,
                Err(_) => String::from(""),
            }
        }

        fn build_pre_tag(&self, _attributes: &HashMap<String, String>) -> String {
            String::from("")
        }

        fn build_code_tag(&self, _attributes: &HashMap<String, String>) -> String {
            String::from("")
        }
    }

    let adapter = MockAdapter {};
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let head = make_head(options);
    let footer = make_footer();
    let css_options = grass::Options::default();

    let css = grass::from_path(
        "src/css/style.scss",
        &css_options.style(OutputStyle::Compressed),
    )
    .unwrap_or_else(|_| "There was a problem parsing the CSS".to_string());

    let body = markdown_to_html_with_plugins(md, &comrak_options, &plugins);
    let html = format!("{}{}{}", head, body, footer);

    Site { html, css }
}
