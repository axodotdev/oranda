use crate::config::Config;
use crate::data::workspaces::WorkspaceData;
use crate::errors::Result;
use crate::site::link::determine_path;
use crate::site::markdown::to_html;
use axoasset::LocalAsset;
use camino::Utf8PathBuf;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct WorkspaceIndexContext {
    pub members: Vec<WorkspaceIndexMember>,
    pub docs_content: Option<String>,
    pub preferred_members: Vec<WorkspaceIndexMember>,
}

#[derive(Serialize, Debug)]
pub struct WorkspaceIndexMember {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub logo: Option<Utf8PathBuf>,
}

impl WorkspaceIndexContext {
    pub fn new(members: &Vec<WorkspaceData>, workspace_config: &Config) -> Result<Self> {
        let mut index_members = Vec::new();
        let mut index_preferred_members = Vec::new();

        for member in members {
            let logo = if let Some(logo) = &member.config.styles.logo {
                Some(Self::find_logo_path(logo, member, workspace_config)?)
            } else {
                None
            };
            let context = WorkspaceIndexMember {
                name: member.config.project.name.clone(),
                slug: member.slug.clone(),
                description: member.config.project.description.clone(),
                repository: member.config.project.repository.clone(),
                logo,
            };
            if workspace_config
                .workspace
                .preferred_members
                .contains(&context.slug)
            {
                index_preferred_members.push(context);
            } else {
                index_members.push(context);
            }
        }

        let mut workspace = Self {
            docs_content: None,
            members: index_members,
            preferred_members: index_preferred_members,
        };

        if let Some(docs_path) = &workspace_config.workspace.docs_path {
            let res = LocalAsset::load_string(docs_path)?;
            let html = to_html(&res, &workspace_config.styles.syntax_theme)?;
            workspace.docs_content = Some(html);
        }

        Ok(workspace)
    }

    fn find_logo_path(
        logo_url: &String,
        member: &WorkspaceData,
        workspace_config: &Config,
    ) -> Result<Utf8PathBuf> {
        let root_path = Utf8PathBuf::from_path_buf(std::env::current_dir()?).unwrap_or_default();
        if logo_url.starts_with("http") {
            // Lifted from axoasset. Expose it there?
            let mut filename = url::Url::parse(logo_url)?
                .path()
                .to_string()
                .replace('/', "_");
            filename.remove(0);
            let mut path = Utf8PathBuf::from(
                workspace_config
                    .build
                    .path_prefix
                    .clone()
                    .unwrap_or_default(),
            );
            path.push(&member.slug);
            path.push(filename);
            Ok(path)
        } else {
            determine_path(root_path, &Some(&member.slug), logo_url)
        }
    }
}
