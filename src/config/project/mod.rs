use std::path::PathBuf;

use serde::Deserialize;

mod javascript;
mod rust;

use crate::errors::*;
use crate::message::{Message, MessageType};
use javascript::JavaScript;
use rust::Rust;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
    pub homepage: Option<String>,
}

impl ProjectConfig {
    pub fn load(project_root: Option<PathBuf>) -> Result<Option<ProjectConfig>> {
        if let Some(ptype) = ProjectConfig::detect(&project_root) {
            match ptype {
                ProjectType::JavaScript(project) => Ok(Some(project.read(&project_root)?)),
                ProjectType::Rust(project) => Ok(Some(project.read(&project_root)?)),
            }
        } else {
            Ok(None)
        }
    }

    fn detect(project_root: &Option<PathBuf>) -> Option<ProjectType> {
        if Rust::config(project_root).exists() {
            Message::new(MessageType::Info, "Detected Rust project...").print_and_log();
            Some(ProjectType::Rust(Rust {}))
        } else if JavaScript::config(project_root).exists() {
            Message::new(MessageType::Info, "Detected JavaScript project...").print_and_log();
            Some(ProjectType::JavaScript(JavaScript {}))
        } else {
            Message::new(MessageType::Warning, "Could not detect project type.").print_and_log();
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ProjectType {
    Rust(Rust),
    JavaScript(JavaScript),
}
