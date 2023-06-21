use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyOptExt};

// Social media config (complete version)
#[derive(Debug)]
pub struct SocialConfig {
    pub image: Option<String>,
    pub image_alt: Option<String>,
    pub twitter_account: Option<String>,
}
// Social media config (partial version used by oranda.json)
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SocialLayer {
    pub image: Option<String>,
    pub image_alt: Option<String>,
    pub twitter_account: Option<String>,
}

impl Default for SocialConfig {
    fn default() -> Self {
        SocialConfig {
            image: None,
            image_alt: None,
            twitter_account: None,
        }
    }
}
impl ApplyLayer for SocialConfig {
    type Layer = SocialLayer;
    fn apply_layer(&mut self, layer: Self::Layer) {
        // This is intentionally written slightly cumbersome to make you update this
        let SocialLayer {
            image,
            image_alt,
            twitter_account,
        } = layer;
        self.image.apply_opt(image);
        self.image_alt.apply_opt(image_alt);
        self.twitter_account.apply_opt(twitter_account);
    }
}
