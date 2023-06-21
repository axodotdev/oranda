use crate::config::ApplyLayer;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::site::layout::javascript::analytics::{Fathom, Google, Plausible, Unami};

/// Analytics config (complete version)
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AnalyticsConfig {
    Google(Google),
    Plausible(Plausible),
    Fathom(Fathom),
    Unami(Unami),
}
/// Analytics config (partial version used by oranda.json)
pub type AnalyticsLayer = AnalyticsConfig;

impl ApplyLayer for AnalyticsConfig {
    type Layer = AnalyticsConfig;
    fn apply_layer(&mut self, layer: Self::Layer) {
        // FIXME: this is kinda goofy but there's not an obvious thing to do
        // if we need to change the enum variant and we care about preserving things
        *self = layer;
    }
}
