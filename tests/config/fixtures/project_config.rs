pub fn cargo_toml() -> &'static str {
    r#"
[package]
"name" = "axo"
"version" = "0.0.0"
"description" = "blublublub"
"respository" = "https://github.com/axodotdev/not-a-real-project"
    "#
}

pub fn package_json() -> &'static str {
    r#"
{
    "name": "axo",
    "version": "0.1.0",
    "description": ">o_o<",
    "bin": {
        "axo": "src/main.js"
    },
    "repository": {
        "type": "git",
        "url": "https://github.com/axodotdev/not-a-real-project"
    }
}
    "#
}
