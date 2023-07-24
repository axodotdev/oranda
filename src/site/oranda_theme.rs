use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The serde definitions here define what CSS class these themes lower to
///
/// We're currently transitioning from a system where the light and dark
/// variants of a theme were basically completely independent, to a system
/// where we just use an extra "dark" class to modify the light theme
/// as appropriate.
///
/// So in the fullness of time Light and Dark here should probably get
/// serialized as like "default" and "dark default", just as AxoLight
/// and AxoDark get serialized as "axo" and "dark axo".
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum OrandaTheme {
    Light,
    Dark,
    #[serde(alias = "axo_light", rename(serialize = "axo"))]
    AxoLight,
    #[serde(alias = "axo_dark", rename(serialize = "dark axo"))]
    AxoDark,
    Hacker,
    Cupcake,
}

impl Default for OrandaTheme {
    fn default() -> Self {
        Self::Dark
    }
}
