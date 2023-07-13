use crate::config::FundingConfig;
use crate::data::funding::{Funding, FundingContent, FundingType};
use crate::errors::Result;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct FundingContext {
    preferred_funding: Option<Vec<FundingMethod>>,
    funding: Vec<FundingMethod>,
    docs_content: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct FundingMethod {
    title: String,
    link: String,
    icon: Option<String>,
}

pub fn context(config: &FundingConfig, funding: &Funding) -> Result<FundingContext> {
    let mut funding_base = funding.content.clone();
    // allowed temporarily to preserve the TODO
    #[allow(clippy::manual_map)]
    let preferred_funding = if let Some(preferred) = config.preferred_funding.as_ref() {
        if let Some(content) = funding_base.remove(preferred) {
            Some(to_funding_methods(preferred, &content))
        } else {
            // TODO: check if we warn about this anywhere earlier, the user clearly messed up here
            // and specified a preferred funding that doesn't exist, and should be told about it
            None
        }
    } else {
        None
    };

    Ok(FundingContext {
        preferred_funding,
        funding: funding_base
            .iter()
            .flat_map(|(k, v)| to_funding_methods(k, v))
            .collect(),
        docs_content: funding.docs_content.clone(),
    })
}

fn to_funding_methods(ftype: &FundingType, content: &FundingContent) -> Vec<FundingMethod> {
    let mut return_vec = Vec::new();
    match ftype {
        FundingType::Github => {
            let items = one_or_multiple(content);
            for item in items {
                return_vec.push(FundingMethod {
                    title: "GitHub".to_string(),
                    link: format!("https://github.com/sponsors/{item}"),
                    icon: Some("github".to_string()),
                })
            }
        }
        FundingType::Patreon => {
            if let FundingContent::One(item) = content {
                return_vec.push(FundingMethod {
                    title: "Patreon".to_string(),
                    link: format!("https://patreon.com/{item}"),
                    icon: Some("patreon".to_string()),
                })
            }
        }
        FundingType::OpenCollective => {
            if let FundingContent::One(item) = content {
                return_vec.push(FundingMethod {
                    title: "OpenCollective".to_string(),
                    link: format!("https://opencollective.com/{item}"),
                    icon: Some("opencollective".to_string()),
                })
            }
        }
        FundingType::KoFi => {
            if let FundingContent::One(item) = content {
                return_vec.push(FundingMethod {
                    title: "Ko-fi".to_string(),
                    link: format!("https://ko-fi.com/{item}"),
                    icon: Some("kofi".to_string()),
                })
            }
        }
        FundingType::Tidelift => {
            if let FundingContent::One(item) = content {
                return_vec.push(FundingMethod {
                    title: "Tidelift".to_string(),
                    link: format!("https://tidelift.com/subscription/pkg/{item}"),
                    icon: Some("patreon".to_string()),
                })
            }
        }
        FundingType::CommunityBridge => {
            if let FundingContent::One(item) = content {
                return_vec.push(FundingMethod {
                    title: "CommunityBridge".to_string(),
                    link: format!("https://crowdfunding.lfx.linuxfoundation.org/projects/{item}"),
                    icon: None,
                })
            }
        }
        FundingType::Issuehunt => {
            if let FundingContent::One(item) = content {
                return_vec.push(FundingMethod {
                    title: "IssueHunt".to_string(),
                    link: format!("https://issuehunt.com/r/{item}"),
                    icon: None,
                })
            }
        }
        FundingType::Liberapay => {
            if let FundingContent::One(item) = content {
                return_vec.push(FundingMethod {
                    title: "Liberapay".to_string(),
                    link: format!("https://liberapay.com/{item}"),
                    icon: Some("liberapay".to_string()),
                })
            }
        }
        FundingType::Custom => {
            let items = one_or_multiple(content);
            for item in items {
                return_vec.push(FundingMethod {
                    title: item.clone(),
                    link: item,
                    icon: None,
                })
            }
        }
    }

    return_vec
}

/// Handles either one or multiple funding items, and puts them into a Vec.
fn one_or_multiple(funding: &FundingContent) -> Vec<String> {
    let mut vec = vec![];
    match funding {
        FundingContent::One(item) => vec.push(item.to_owned()),
        FundingContent::Multiple(items) => vec.extend(items.to_vec()),
    }

    vec
}
