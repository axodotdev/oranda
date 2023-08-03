use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
pub enum SyntaxTheme {
    AgilaClassicOceanicNext,
    AgilaCobalt,
    AgilaLightSolarized,
    AgilaMonokaiExtended,
    AgilaNeonMonocyanide,
    AgilaOceanicNext,
    AgilaOriginOceanicNext,
    Base16EightiesDark,
    Base16MochaDark,
    Base16OceanDark,
    Base16OceanLight,
    Darkmatter,
    Dracula,
    GitHubLight,
    MaterialTheme,
    MaterialThemeDarker,
    MaterialThemeLighter,
    MaterialThemePalenight,
    NightOwl,
    OneDark,
}

impl SyntaxTheme {
    pub fn as_str(&self) -> String {
        format!("{:?}", &self)
    }
}
