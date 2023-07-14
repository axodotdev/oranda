use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::{ApplyLayer, ApplyOptExt};

// Social media config (complete version)
#[derive(Debug, Serialize, Clone)]
pub struct SocialConfig {
    pub image: Option<String>,
    pub image_alt: Option<String>,
    pub twitter_account: Option<String>,
}
// Settings for social media integrations
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SocialLayer {
    /// Image to show in link previews
    ///
    /// FIXME: what format?
    pub image: Option<String>,
    /// Alt image to show in link previews
    ///
    /// FIXME: explain the distinction with "image"
    ///
    /// FIXME: what format?
    pub image_alt: Option<String>,
    /// Twitter account to show in link previews
    ///
    /// Example: "@axodotdev"
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
