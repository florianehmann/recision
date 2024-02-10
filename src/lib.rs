use std::{
    fs::{remove_file, File},
    io::{Read, Write},
    path::PathBuf,
};

use ::anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    projects: Vec<Project>,
}

impl Workspace {
    pub fn new() -> Self {
        return Self {
            projects: Vec::new(),
        };
    }

    pub fn add_project(&mut self, project: Project) -> &mut Self {
        self.projects.push(project);
        return self;
    }

    pub fn write_to_file(&self, path: PathBuf) -> Result<()> {
        if path.exists() {
            remove_file(path.clone()).with_context(|| "removing existing worspace file")?;
        }

        let toml_string = toml::to_string_pretty(self)?;
        let mut file = File::create(path.clone()).with_context(|| "creating new workspace file")?;
        file.write_all(toml_string.as_bytes())?;

        return Ok(());
    }

    pub fn read_from_file(path: PathBuf) -> Result<Self> {
        let mut file = File::open(path.clone()).with_context(|| "opening workspace file")?;
        let mut toml_string = String::new();
        file.read_to_string(&mut toml_string)
            .with_context(|| "reading workspace from file")?;
        let workspace = toml::from_str(&toml_string.as_str())
            .with_context(|| "parsing contents wof workspace file")?;

        return Ok(workspace);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
}

impl Project {
    pub fn new(name: &str) -> Self {
        return Self {
            name: String::from(name),
        };
    }
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use crate::*;

    fn build_test_workspace() -> Workspace {
        let mut workspace = Workspace::new();

        workspace
            .add_project(Project::new("Project 1"))
            .add_project(Project::new("Project 2"))
            .add_project(Project::new("Project 3"))
            .add_project(Project::new("Project ="))
            .add_project(Project::new("Project [toml]"))
            .add_project(Project::new("Project\nNewline"));

        return workspace;
    }

    #[test]
    fn test() {
        let _ = build_test_workspace();
    }

    #[test]
    fn test_write_workspace_to_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let workspace = build_test_workspace();
        workspace
            .write_to_file(temp_file.path().to_path_buf())
            .unwrap();
    }

    #[test]
    fn test_reconstruct_from_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let workspace = build_test_workspace();
        workspace
            .write_to_file(temp_file.path().to_path_buf())
            .unwrap();

        let reconstructed_workspace = Workspace::read_from_file(temp_file.path().to_path_buf())
            .unwrap();

        println!("{:?}", reconstructed_workspace);
    }
}
