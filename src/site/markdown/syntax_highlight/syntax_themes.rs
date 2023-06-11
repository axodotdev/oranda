use serde::Deserialize;
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
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
