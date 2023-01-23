use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub enum SyntaxThemes {
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

impl SyntaxThemes {
    pub fn as_str(&self) -> String {
        format!("{:?}", &self)
    }
}
