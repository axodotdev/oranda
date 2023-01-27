use std::path::PathBuf;

use serde::Deserialize;

mod javascript;
mod rust;

use crate::errors::*;
use crate::message::{Message, MessageType};

pub use javascript::JavaScript;
pub use rust::Rust;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
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
            Message::new(MessageType::Info, "Detected Rust project...").print();
            tracing::info!("Detected Rust project...");
            Some(Type::Rust(Rust {}))
        } else if JavaScript::config(project_root).exists() {
            Message::new(MessageType::Info, "Detected JavaScript project...").print();
            tracing::info!("Detected JavaScript project.");
            Some(Type::JavaScript(JavaScript {}))
        } else {
            Message::new(MessageType::Warning, "Could not identify project type...").print();
            tracing::warn!("Could not identify project type...");
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Rust(Rust),
    JavaScript(JavaScript),
}
