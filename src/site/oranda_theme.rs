use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Themes for oranda's output
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum OrandaTheme {
    Light,
    Dark,
    #[serde(alias = "axo_light")]
    AxoLight,
    #[serde(alias = "axo_dark")]
    AxoDark,
    Hacker,
    Cupcake,
}

impl Default for OrandaTheme {
    fn default() -> Self {
        Self::Dark
    }
}
