use crate::data::workspaces::WorkspaceData;
use indexmap::IndexMap;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct WorkspaceIndexContext {
    pub members: IndexMap<String, WorkspaceIndexMember>,
}

#[derive(Serialize, Debug)]
pub struct WorkspaceIndexMember {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

impl WorkspaceIndexContext {
    pub fn new(members: &Vec<WorkspaceData>) -> Self {
        let mut map = IndexMap::new();
        for member in members {
            let context = WorkspaceIndexMember {
                name: member.config.project.name.clone(),
                slug: member.slug.clone(),
                description: member.config.project.description.clone(),
            };
            map.insert(member.slug.clone(), context);
        }

        Self { members: map }
    }
}
