use schemars::JsonSchema;
use serde::Deserialize;

use crate::site::layout::javascript::analytics::{Fathom, Google, Plausible, Umami};

/// Settings for Analytics
///
/// Analytics providers are currently mututally exclusive -- you can pick at most one.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AnalyticsConfig {
    /// Use Google Analytics
    Google(Google),
    /// Use Plausible Analytics
    Plausible(Plausible),
    /// Use Fathom Analytics
    Fathom(Fathom),
    /// Use Umami Analytics
    Umami(Umami),
}
