use crate::config::ApplyLayer;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::site::layout::javascript::analytics::{Fathom, Google, Plausible, Unami};

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AnalyticsConfig {
    Google(Google),
    Plausible(Plausible),
    Fathom(Fathom),
    Unami(Unami),
}
impl ApplyLayer for AnalyticsConfig {
    fn apply_layer(&mut self, layer: Self) {
        // FIXME: this is kinda goofy but there's not an obvious thing to do
        // if we need to change the enum variant and we care about preserving things
        *self = layer;
    }
}
