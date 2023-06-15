use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyOptExt};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SocialConfig {
    pub image: Option<String>,
    pub image_alt: Option<String>,
    pub twitter_account: Option<String>,
}

impl ApplyLayer for SocialConfig {
    fn apply_layer(&mut self, layer: Self) {
        self.image.apply_opt(layer.image);
        self.image_alt.apply_opt(layer.image_alt);
        self.twitter_account.apply_opt(layer.twitter_account);
    }
}
