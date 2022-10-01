use crate::config::{theme, Config};

pub fn make_head(config: &Config) -> String {
    format!(
        r#"
   <!DOCTYPE html>
   <html lang="en">
   <head>
   
   <meta charset="utf-8">
   <meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1" />
   <meta name="viewport" content="width=device-width, initial-scale=1.0">
   <link rel="stylesheet" href="styles.css"></link>
   
   <title>{name}</title>
   <meta name="description" content={description} />
    <meta property="og:url" content={homepage}>
   </head>
   <body>
   <div id="oranda"><div class="body {theme}"><div class="container">
   "#,
        name = &config.name,
        description = &config.description,
        theme = theme::css_class(&config.theme),
        homepage = &config.homepage.as_ref().unwrap(),
    )
}
