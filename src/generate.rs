use crate::errors::Result;
use axoasset::LocalAsset;
use camino::Utf8PathBuf;
use inquire::ui::{Color, RenderConfig, Styled};
use inquire::Confirm;
use minijinja::{context, Environment};

const CI_TEMPLATE: &str = include_str!("../templates/generate/web.yml.j2");

pub fn generate_ci(path: Utf8PathBuf, site_dir: &Option<Utf8PathBuf>) -> Result<()> {
    tracing::info!("Generating a CI deploy workflow for your site...");

    let mut env = Environment::new();
    // Modify the syntax so that it doesn't clash with GitHub Actions' syntax.
    env.set_syntax(minijinja::Syntax {
        block_start: "{{%".into(),
        block_end: "%}}".into(),
        variable_start: "{{{".into(),
        variable_end: "}}}".into(),
        comment_start: "{{#".into(),
        comment_end: "#}}".into(),
    })?;
    env.add_template_owned("web.yml", CI_TEMPLATE)?;
    let prompt_prefix = Styled::new("? >o_o<").with_fg(Color::DarkGreen);
    let render_config = RenderConfig::default().with_prompt_prefix(prompt_prefix);

    // Does the file already exist? If so, prompt the user to overwrite.
    let existing_file = if path.exists() {
        let confirm_prompt = Confirm::new(&format!(
            "There's already a file at {:?}! Do you want to override it?",
            path
        ))
        .with_default(false)
        .with_render_config(render_config)
        .prompt();

        if let Ok(false) = confirm_prompt {
            tracing::info!("Ok, exiting...");
            return Ok(());
        }
        let file = LocalAsset::load_string(&path)?;
        Some(file)
    } else {
        None
    };

    // Ask whether we want to include a link checker
    let check_links_prompt =
        Confirm::new("Do you want to include a tool for checking generated hyperlinks?")
            .with_default(true)
            .with_help_message(
                "Read more about the link checker tool: https://github.com/untitaker/hyperlink",
            )
            .with_render_config(render_config)
            .prompt()
            .expect("Error while prompting!");

    let use_latest_oranda_prompt = Confirm::new("Do you want to always use the latest version of oranda?")
        .with_default(false)
        .with_help_message("Using the latest version means you don't have to rerun this command when oranda updates, but it may also break your site if you use a lot of configuration!")
        .with_render_config(render_config)
        .prompt()
        .expect("Error while prompting!");

    let context = context!(check_links => check_links_prompt, use_latest_oranda => use_latest_oranda_prompt, site_dir => site_dir);
    let template = env.get_template("web.yml")?;
    let rendered = template.render(context)?;
    if existing_file.is_some_and(|file| file == rendered) {
        tracing::warn!("File exists and is identical, aborting...");
        return Ok(());
    }
    LocalAsset::write_new_all(&rendered, &path)?;
    tracing::info!(success = true, "Wrote CI file to {:?}", path);

    Ok(())
}
