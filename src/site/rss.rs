use crate::config::Config;
use crate::errors::Result;
use crate::site::changelog::ChangelogContext;
use crate::site::link::generate_absolute;
use rss::extension::atom;
use rss::{CategoryBuilder, Channel, ChannelBuilder, GuidBuilder, Item, ItemBuilder};

pub fn generate_rss_feed(context: &ChangelogContext, config: &Config) -> Result<Channel> {
    let category = CategoryBuilder::default()
        .name(format!("{} Changelog", &config.project.name))
        .domain(config.project.repository.clone())
        .build();

    let mut items: Vec<Item> = Vec::new();
    for release in &context.releases {
        let link =
            generate_absolute(config, &format!("changelog/{}", release.version_tag)).unwrap();
        let guid = GuidBuilder::default().permalink(true).value(&link).build();
        let item = ItemBuilder::default()
            .title(release.name.clone().unwrap_or(release.version_tag.clone()))
            .content(Some(release.body.clone()))
            .categories(vec![category.clone()])
            .link(link)
            .guid(guid)
            .build();
        items.push(item);
    }

    let self_link = atom::Link {
        rel: "self".to_string(),
        href: generate_absolute(config, "changelog.rss").unwrap(),
        ..Default::default()
    };
    let atom_link = atom::AtomExtensionBuilder::default()
        .links(vec![self_link])
        .build();
    let channel = ChannelBuilder::default()
        .title(format!("{} Changelog", &config.project.name))
        .description(format!(
            "Changelog information for {}",
            &config.project.name
        ))
        .categories(vec![category])
        .items(items)
        .link(generate_absolute(config, "changelog").unwrap())
        .atom_ext(atom_link)
        .build();
    Ok(channel)
}
