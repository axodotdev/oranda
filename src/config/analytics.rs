use crate::config::Config;
use axohtml::elements::script;

use axohtml::{html, text};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoogleTracking {
    pub tracking_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FathomTracking {
    pub site: String,
}

#[derive(Debug, Deserialize)]
pub struct PlausibleTracking {
    pub domain: String,
    pub script_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UnamiTracking {
    pub website: String,
    pub script_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Analytics {
    Google(GoogleTracking),
    Plausible(PlausibleTracking),
    Fathom(FathomTracking),
    Unami(UnamiTracking),
}

const GOOGLE_SCRIPT_URL: &str = "https://www.googletagmanager.com/gtag/js";
const PLAUSIBLE_SCRIPT_URL: &str = "https://plausible.io/js/script.js";
const FATHOM_SCRIPT_URL: &str = "https://cdn.usefathom.com/script.js";

pub fn get_google_script(config: &Config) -> Option<Box<script<String>>> {
    let analytics = config.analytics.as_ref();

    match analytics {
        None => None,
        Some(Analytics::Google(g)) => {
            let code = text!("window.dataLayer = window.dataLayer || []; function gtag(){{dataLayer.push(arguments);}} gtag('js', new Date());gtag('config', {});", g.tracking_id);

            Some(html!(<script>{code}</script>))
        }
        Some(_) => None,
    }
}
pub fn get_analytics(config: &Config) -> Option<Box<script<String>>> {
    let analytics = config.analytics.as_ref();

    match analytics {
        None => None,
        Some(Analytics::Fathom(f)) => {
            Some(html!(<script defer=true src=FATHOM_SCRIPT_URL data-site=&f.site ></script>))
        }
        Some(Analytics::Unami(u)) => Some(
            html!(<script async=true defer=true src=&u.script_url data-website-id=&u.website></script>),
        ),
        Some(Analytics::Google(g)) => {
            let script_url = format!("{}?id={}", GOOGLE_SCRIPT_URL, g.tracking_id);
            Some(html!(
                <script async=true src=&script_url></script>
            ))
        }
        Some(Analytics::Plausible(p)) => {
            let url = PLAUSIBLE_SCRIPT_URL.to_string();
            let script_url = p.script_url.as_ref().unwrap_or(&url);
            Some(html!(
                <script defer=true data-domain=&p.domain src=script_url></script>
            ))
        }
    }
}
