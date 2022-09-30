use crate::options::{theme, Options};

pub fn make_head(options: &Options) -> String {
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
        name = &options.name,
        description = &options.description,
        theme = theme::css_class(&options.theme),
        homepage = &options.homepage.as_ref().unwrap(),
    )
}
