use schemars::JsonSchema;
use serde::Deserialize;

use crate::site::layout::javascript::analytics::{Fathom, Google, Plausible, Unami};

/// Analytics config (complete version, but also partial oranda.json version)
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AnalyticsConfig {
    Google(Google),
    Plausible(Plausible),
    Fathom(Fathom),
    Unami(Unami),
}
