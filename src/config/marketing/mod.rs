pub use analytics::{AnalyticsConfig, AnalyticsLayer};
use schemars::JsonSchema;
use serde::Deserialize;
pub use social::{SocialConfig, SocialLayer};

use super::ApplyLayer;

mod analytics;
mod social;

/// Marketing config
#[derive(Debug)]
pub struct MarketingConfig {
    /// Analytics
    pub analytics: Option<AnalyticsConfig>,
    /// Social media
    pub social: Option<SocialConfig>,
}
/// Marketing config
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MarketingLayer {
    /// Analytics
    pub analytics: Option<AnalyticsLayer>,
    /// Social media
    pub social: Option<SocialLayer>,
}

impl Default for MarketingConfig {
    fn default() -> Self {
        MarketingConfig {
            analytics: None,
            social: None,
        }
    }
}
impl ApplyLayer for MarketingConfig {
    type Layer = MarketingLayer;
    fn apply_layer(&mut self, layer: Self::Layer) {
        // This is intentionally written slightly cumbersome to make you update this
        let MarketingLayer { analytics, social } = layer;
        self.analytics.apply_layer(analytics);
        self.social.apply_layer(social);
    }
}
