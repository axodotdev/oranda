use crate::config::Config;
use crate::errors::*;
use axohtml::elements::script;

use axohtml::html;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GoogleTracking {
    pub tracking_id: String,
}

#[derive(Debug, Deserialize)]
struct FathomTracking {
    pub site: String,
}

#[derive(Debug, Deserialize)]
struct PlausibleTracking {
    pub domain: String,
    pub script_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UnamiTracking {
    pub website: String,
    pub script_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Analytics {
    google: Option<GoogleTracking>,
    plausible: Option<PlausibleTracking>,
    fathom: Option<FathomTracking>,
    unami: Option<UnamiTracking>,
}

const GOOGLE_SCRIPT_URL: &str = "https://www.googletagmanager.com/gtag/js";
const PLAUSIBLE_SCRIPT_URL: &str = "https://plausible.io/js/script.js";
const FATHOM_SCRIPT_URL: &str = "https://cdn.usefathom.com/script.js";

pub fn get_google_script(config: &Config) -> Box<script<String>> {
    return config
        .analytics
        .as_ref()
        .map(|analytics_type| {
            analytics_type
                .google
                .as_ref()
                .map(|g| html!(<script>{format!("window.dataLayer = window.dataLayer || []; function gtag(){{dataLayer.push(arguments);}} gtag('js', new Date());gtag('config', {});", g.tracking_id)}</script>))
        })
        .unwrap()
        .unwrap();
}
pub fn get_analytics(config: &Config) -> Result<Box<script<String>>> {
    let analytics = config.analytics.as_ref();
    let script = analytics.map(|analytics_type| {
        let fathom = analytics_type.fathom.as_ref();
        let plausible = analytics_type.plausible.as_ref();
        let google = analytics_type.google.as_ref();
        let unami = analytics_type.unami.as_ref();

        if fathom.is_none() && plausible.is_none() && google.is_none() && unami.is_none() {
            return Err(OrandaError::Other(
                "Invalid configuration, you need to choose an analytics type".to_string(),
            ));
        }

        if fathom.is_some() {
            let site = fathom.unwrap();
            return Ok(html!(
                <script defer=true src=FATHOM_SCRIPT_URL data-site=&site.site ></script>
            ));
        }

        if unami.is_some() {
            let site = unami.unwrap();
            return Ok(html!(
                <script
                    async=true
                    defer=true
                    src=&site.script_url
                    data-website-id=&site.website
                ></script>
            ));
        }

        if google.is_some() {
            let site = google.unwrap();
            let script_url = format!("{}?id={}", GOOGLE_SCRIPT_URL, site.tracking_id);
            return Ok(html!(
                <script async=true src=&script_url></script>

            ));
        }

        let site = plausible.unwrap();
        let url = PLAUSIBLE_SCRIPT_URL.to_string();
        let script_url = site.script_url.as_ref().unwrap_or(&url);
        return Ok(html!(
            <script defer=true data-domain=&site.domain src=script_url></script>

        ));
    });

    return script.unwrap();
}
