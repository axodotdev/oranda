use minijinja::context;
use oranda::config::Config;
use oranda::data::Context;
use oranda::site::templates::Templates;
use oranda::site::{self, artifacts, changelog, page::Page};

fn reset(dist_dir: &str) {
    site::Site::clean_dist_dir(dist_dir).unwrap();
}

pub fn index(config: &Config) -> Page {
    reset(&config.build.dist_dir);
    let templates = Templates::new(config).unwrap();
    Page::new_from_both(
        &config.project.readme_path,
        "index.html",
        &templates,
        "index.html",
        context!(),
        config,
    )
    .unwrap()
}

pub fn index_with_artifacts(config: &Config) -> Page {
    reset(&config.build.dist_dir);
    let templates = Templates::new(config).unwrap();
    let repo_url = config.project.repository.as_ref().unwrap();
    let mut context = Context::new_github(
        repo_url,
        &config.project,
        config.components.artifacts.as_ref(),
    )
    .unwrap();
    if let Some(latest) = context.latest_mut() {
        latest.artifacts.make_scripts_viewable(config).unwrap();
    }
    let template_context = artifacts::template_context(&context, config).unwrap();
    Page::new_from_both(
        &config.project.readme_path,
        "index.html",
        &templates,
        "index.html",
        context!(artifacts => template_context),
        config,
    )
    .unwrap()
}

pub fn artifacts(config: &Config) -> Page {
    reset(&config.build.dist_dir);
    let templates = Templates::new(config).unwrap();
    let repo_url = config.project.repository.as_ref().unwrap();
    let context = Context::new_github(
        repo_url,
        &config.project,
        config.components.artifacts.as_ref(),
    )
    .unwrap();
    let template_context = artifacts::template_context(&context, config).unwrap();
    Page::new_from_template(
        "artifacts.html",
        &templates,
        "artifacts.html",
        &template_context,
    )
    .unwrap()
}

pub fn changelog(config: &Config) -> Page {
    reset(&config.build.dist_dir);
    let templates = Templates::new(config).unwrap();
    let repo_url = config.project.repository.as_ref().unwrap();
    let context = Context::new_github(
        repo_url,
        &config.project,
        config.components.artifacts.as_ref(),
    )
    .unwrap();
    let index_context = changelog::index_context(&context, config).unwrap();
    Page::new_from_template(
        "changelog.html",
        &templates,
        "changelog_index.html",
        index_context,
    )
    .unwrap()
}
