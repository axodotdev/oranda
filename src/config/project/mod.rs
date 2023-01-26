use std::path::PathBuf;

use serde::Deserialize;

mod javascript;
mod rust;

use crate::errors::*;
pub use javascript::JavaScript;
pub use rust::Rust;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub version: Option<String>,
}

impl ProjectConfig {
    pub fn load(project_root: Option<PathBuf>) -> Result<Option<ProjectConfig>> {
        if let Some(ptype) = ProjectConfig::detect(&project_root) {
            match ptype {
                Type::JavaScript(project) => Ok(Some(project.read(&project_root)?)),
                Type::Rust(project) => Ok(Some(project.read(&project_root)?)),
            }
        } else {
            Ok(None)
        }
    }

    pub fn detect(project_root: &Option<PathBuf>) -> Option<Type> {
        if Rust::config(project_root).exists() {
            Some(Type::Rust(Rust {}))
        } else if JavaScript::config(project_root).exists() {
            Some(Type::JavaScript(JavaScript {}))
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Rust(Rust),
    JavaScript(JavaScript),
}
