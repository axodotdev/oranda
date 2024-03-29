use crate::config::Config;
use crate::errors::{OrandaError, Result};
use camino::Utf8PathBuf;

#[derive(Debug, Clone)]
pub struct WorkspaceData {
    pub root_path: Utf8PathBuf,
    pub slug: String,
    pub path: Utf8PathBuf,
    pub config: Config,
}

pub fn from_config(
    workspace_config: &Config,
    root_path: &Utf8PathBuf,
    workspace_config_path: &Utf8PathBuf,
) -> Result<Vec<WorkspaceData>> {
    let mut vec = Vec::new();
    for member in workspace_config.workspace.members.clone() {
        if !member.path.exists() {
            return Err(OrandaError::FileNotFound {
                filedesc: "workspace member".to_string(),
                path: member.path.display().to_string(),
            });
        }

        // FIXME: I expect this to break at some point, because making paths absolute is an absolute
        // hellhole, and should not be taken for granted.
        let path = Utf8PathBuf::from(member.path.display().to_string()).canonicalize_utf8()?;
        let mut config_path = path.clone();
        config_path.push("oranda.json");
        let mut config = Config::build_workspace_member(
            &config_path,
            workspace_config_path,
            &path,
            &member,
            Some(member.slug.clone()),
        )?;

        // Set the correct path prefix. This should be:
        // - If no root path prefix: `slug`
        // - If root path prefix: `path_prefix/slug`
        config.build.path_prefix =
            if let Some(path_prefix) = workspace_config.build.path_prefix.as_ref() {
                // FIXME: Doesn't account for trailing slashes right now
                Some(format!("{}/{}", path_prefix, &member.slug))
            } else {
                Some(member.slug.to_string())
            };

        // Set the correct dist_dir. This should be `cwd_from_root/workspace_dist_dir/slug`
        config.build.dist_dir = format!(
            "{}/{}/{}",
            root_path, workspace_config.build.dist_dir, &member.slug
        );

        vec.push(WorkspaceData {
            root_path: root_path.clone(),
            slug: member.slug.clone(),
            path,
            config,
        });
    }

    Ok(vec)
}
