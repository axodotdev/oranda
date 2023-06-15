use crate::config::ApplyLayer;
use axohtml::elements::script;

use axohtml::{html, unsafe_text};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GoogleTracking {
    pub tracking_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FathomTracking {
    pub site: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PlausibleTracking {
    pub domain: String,
    pub script_url: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnamiTracking {
    pub website: String,
    pub script_url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AnalyticsConfig {
    Google(GoogleTracking),
    Plausible(PlausibleTracking),
    Fathom(FathomTracking),
    Unami(UnamiTracking),
}
impl ApplyLayer for AnalyticsConfig {
    fn apply_layer(&mut self, layer: Self) {
        // FIXME: this is kinda goofy but there's not an obvious thing to do
        // if we need to change the enum variant and we care about preserving things
        *self = layer;
    }
}

const GOOGLE_SCRIPT_URL: &str = "https://www.googletagmanager.com/gtag/js";
const PLAUSIBLE_SCRIPT_URL: &str = "https://plausible.io/js/script.js";
const FATHOM_SCRIPT_URL: &str = "https://cdn.usefathom.com/script.js";

impl GoogleTracking {
    pub fn get_script(&self) -> Box<script<String>> {
        let code = format!("window.dataLayer = window.dataLayer || []; function gtag(){{dataLayer.push(arguments);}} gtag('js', new Date());gtag('config', '{}');", self.tracking_id);

        html!(<script>{unsafe_text!(code)}</script>)
    }
}

impl AnalyticsConfig {
    pub fn snippet(&self) -> Box<script<String>> {
        match self {
            AnalyticsConfig::Fathom(f) => {
                html!(<script defer=true src=FATHOM_SCRIPT_URL data-site=&f.site ></script>)
            }
            AnalyticsConfig::Unami(u) => {
                html!(<script async=true defer=true src=&u.script_url data-website-id=&u.website></script>)
            }
            AnalyticsConfig::Google(g) => {
                let script_url = format!("{}?id={}", GOOGLE_SCRIPT_URL, g.tracking_id);
                html!(
                    <script async=true src=&script_url></script>
                )
            }
            AnalyticsConfig::Plausible(p) => {
                let url = PLAUSIBLE_SCRIPT_URL.to_string();
                let script_url = p.script_url.as_ref().unwrap_or(&url);
                html!(
                    <script defer=true data-domain=&p.domain src=script_url></script>
                )
            }
        }
    }
}
