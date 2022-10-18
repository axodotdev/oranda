use std::path::PathBuf;

use serde::Deserialize;

mod javascript;
mod rust;

use crate::errors::*;
use javascript::JavaScript;
use rust::Rust;

#[derive(Debug, Deserialize, PartialEq)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
    pub homepage: Option<String>,
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

    fn detect(project_root: &Option<PathBuf>) -> Option<Type> {
        if Rust::config(&project_root).exists() {
            Some(Type::Rust(Rust {}))
        } else if JavaScript::config(&project_root).exists() {
            Some(Type::JavaScript(JavaScript {}))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Rust(Rust),
    JavaScript(JavaScript),
}
