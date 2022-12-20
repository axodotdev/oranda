use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
    pub fn as_str(&self) -> &'static str {
        match self {
            SyntaxThemes::AgilaClassicOceanicNext => "AgilaClassicOceanicNext",
            SyntaxThemes::AgilaCobalt => "AgilaCobalt",
            SyntaxThemes::AgilaLightSolarized => "AgilaLightSolarized",
            SyntaxThemes::AgilaMonokaiExtended => "AgilaMonokaiExtended",
            SyntaxThemes::AgilaNeonMonocyanide => "AgilaNeonMonocyanide",
            SyntaxThemes::AgilaOceanicNext => "AgilaOceanicNext",
            SyntaxThemes::AgilaOriginOceanicNext => "AgilaOriginOceanicNext",
            SyntaxThemes::Base16EightiesDark => "Base16EightiesDark",
            SyntaxThemes::Base16MochaDark => "Base16MochaDark",
            SyntaxThemes::Base16OceanDark => "Base16OceanDark",
            SyntaxThemes::Base16OceanLight => "Base16OceanLight",
            SyntaxThemes::Darkmatter => "Darkmatter",
            SyntaxThemes::Dracula => "Dracula",
            SyntaxThemes::GitHubLight => "GitHubLight",
            SyntaxThemes::MaterialTheme => "MaterialTheme",
            SyntaxThemes::MaterialThemeDarker => "MaterialThemeDarker",
            SyntaxThemes::MaterialThemeLighter => "MaterialThemeLighter",
            SyntaxThemes::MaterialThemePalenight => "MaterialThemePalenight",
            SyntaxThemes::NightOwl => "NightOwl",
            SyntaxThemes::OneDark => "OneDark",
        }
    }
}
