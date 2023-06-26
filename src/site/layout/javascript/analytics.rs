use axohtml::elements::script;
use axohtml::{html, unsafe_text};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::AnalyticsConfig;

pub struct Analytics {
    pub snippet: Option<Box<script<String>>>,
    pub google_script: Option<Box<script<String>>>,
}

impl Analytics {
    pub fn new(config: &Option<AnalyticsConfig>) -> Self {
        if let Some(analytics) = config {
            match analytics {
                AnalyticsConfig::Google(provider) => {
                    let google_script = Some(provider.get_script());
                    Self {
                        snippet: Some(provider.snippet()),
                        google_script,
                    }
                }
                AnalyticsConfig::Plausible(provider) => Self::build(provider),
                AnalyticsConfig::Fathom(provider) => Self::build(provider),
                AnalyticsConfig::Umami(provider) => Self::build(provider),
            }
        } else {
            Self {
                snippet: None,
                google_script: None,
            }
        }
    }

    fn build<T: Snippet>(provider: &T) -> Self {
        Self {
            snippet: Some(provider.snippet()),
            google_script: None,
        }
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Google {
    pub tracking_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Fathom {
    pub site: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Plausible {
    pub domain: String,
    pub script_url: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Umami {
    pub website: String,
    pub script_url: String,
}

const GOOGLE_SCRIPT_URL: &str = "https://www.googletagmanager.com/gtag/js";
const PLAUSIBLE_SCRIPT_URL: &str = "https://plausible.io/js/script.js";
const FATHOM_SCRIPT_URL: &str = "https://cdn.usefathom.com/script.js";

impl Google {
    pub fn get_script(&self) -> Box<script<String>> {
        let code = format!("window.dataLayer = window.dataLayer || []; function gtag(){{dataLayer.push(arguments);}} gtag('js', new Date());gtag('config', '{}');", self.tracking_id);

        html!(<script>{unsafe_text!(code)}</script>)
    }
}

trait Snippet {
    fn snippet(&self) -> Box<script<String>>;
}

impl Snippet for Google {
    fn snippet(&self) -> Box<script<String>> {
        let script_url = format!("{}?id={}", GOOGLE_SCRIPT_URL, self.tracking_id);
        html!(
            <script async=true src=&script_url></script>
        )
    }
}

impl Snippet for Fathom {
    fn snippet(&self) -> Box<script<String>> {
        html!(<script defer=true src=FATHOM_SCRIPT_URL data-site=&self.site ></script>)
    }
}

impl Snippet for Umami {
    fn snippet(&self) -> Box<script<String>> {
        html!(<script async=true defer=true src=&self.script_url data-website-id=&self.website></script>)
    }
}

impl Snippet for Plausible {
    fn snippet(&self) -> Box<script<String>> {
        let url = PLAUSIBLE_SCRIPT_URL.to_string();
        let script_url = self.script_url.as_ref().unwrap_or(&url);
        html!(
            <script defer=true data-domain=&self.domain src=script_url></script>
        )
    }
}
