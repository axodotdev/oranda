mod icons;
mod links;

use crate::{errors::*, site::repo};
use axohtml::{html, text};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contents {
    pub name: String,
    pub content: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Funding {
    github: Option<Vec<String>>,
    patreon: Option<String>,
    open_collective: Option<String>,
    ko_fi: Option<String>,
    tidelift: Option<String>,
    community_bridge: Option<String>,
    liberapay: Option<String>,
    issuehunt: Option<String>,
    lfx_crowdfunding: Option<String>,
    custom: Option<Vec<String>>,
}

pub fn parse_funding_yaml(content: String) -> Result<Funding> {
    let string_yaml = &general_purpose::STANDARD
        .decode(content.replace('\n', ""))
        .unwrap();
    match std::str::from_utf8(string_yaml) {
        Ok(parsed_yaml) => {
            let deserialized_map = serde_yaml::from_str(parsed_yaml);

            match deserialized_map {
                Ok(yaml) => Ok(yaml),
                Err(e) => Err(OrandaError::GithubFundingParseError {
                    details: e.to_string(),
                }),
            }
        }
        Err(e) => Err(OrandaError::GithubFundingParseError {
            details: e.to_string(),
        }),
    }
}

pub fn fetch_funding_info(repo: &str) -> Result<String> {
    let url_parts = repo::parse(repo)?;
    let response = repo::fetch_funding_file(url_parts)?;
    match response.json::<Contents>() {
        Ok(contents) => {
            let funding_yaml = parse_funding_yaml(contents.content)?;
            let funding_html = links::build(funding_yaml);
            Ok(html!(
                <div class="funding-wrapper">
                    <h4>{text!("Help fund this project")}</h4>
                    <ul class="funding-list">
                        {funding_html}
                    </ul>
                </div>
            )
            .to_string())
        }
        Err(e) => Err(OrandaError::GithubFundingParseError {
            details: e.to_string(),
        }),
    }
}
