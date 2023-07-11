use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
