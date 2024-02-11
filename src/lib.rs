use std::{
    collections::BTreeMap,
    fs::{remove_file, File},
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    projects: Vec<Project>,
    criteria: Vec<Criterion>,
    priority_sets: Vec<PrioritySet>,
    active_priority_set: Option<PrioritySet>,
}

impl Workspace {
    pub fn new() -> Self {
        return Self {
            projects: Vec::new(),
            criteria: Vec::new(),
            priority_sets: Vec::new(),
            active_priority_set: None,
        };
    }

    pub fn add_project(&mut self, project: Project) -> &mut Self {
        self.projects.push(project);
        return self;
    }

    pub fn add_criterion(&mut self, criterion: Criterion) -> &mut Self {
        self.criteria.push(criterion);
        return self;
    }

    pub fn get_project(&mut self, name: &str) -> Option<&mut Project> {
        self.projects
            .iter_mut()
            .filter(|project| project.name == name)
            .next()
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

    pub fn calculate_score(&self) -> Option<BTreeMap<Project, f64>> {
        let Some(priority_set) = self.active_priority_set.as_ref() else {
            return None;
        };

        let mut result = BTreeMap::new();
        self.projects.iter().for_each(|project| {
            let score = project.calculate_score(&self.criteria, &priority_set);
            result.insert(project.clone(), score);
        });

        return Some(result);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Project {
    name: String,
    weights: BTreeMap<Criterion, i32>,
}

impl Project {
    pub fn new(name: &str) -> Self {
        return Self {
            name: String::from(name),
            weights: BTreeMap::new(),
        };
    }

    fn calculate_score(&self, criteria: &Vec<Criterion>, priority_set: &PrioritySet) -> f64 {
        let mut score = 0.0;
        criteria.iter().for_each(|criterion| {
            // TODO define default priority in the configuration
            let priority = priority_set
                .priorities
                .get(criterion)
                .or(Some(&1.0))
                .unwrap();
            let weight = self.weights.get(criterion).or(Some(&0)).unwrap();

            score += *weight as f64 * priority;
        });

        return score;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Criterion {
    name: String,
}

impl Criterion {
    pub fn new(name: &str) -> Self {
        return Self {
            name: String::from(name),
        };
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrioritySet {
    name: String,
    priorities: BTreeMap<Criterion, f64>,
}

impl PrioritySet {
    pub fn new(name: &str) -> Self {
        return Self {
            name: String::from(name),
            priorities: BTreeMap::new(),
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
            .add_project(Project::new("Project ="))
            .add_project(Project::new("Project [toml]"))
            .add_project(Project::new("Project\nNewline"));

        workspace
            .add_criterion(Criterion::new("Fun"))
            .add_criterion(Criterion::new("Useful"));

        // TODO adjust weights
        let mut proj1 = workspace.get_project("Project 1");

        return workspace;
    }

    #[test]
    fn test_buildin_workpace() {
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

        let reconstructed_workspace =
            Workspace::read_from_file(temp_file.path().to_path_buf()).unwrap();

        assert_eq!(workspace, reconstructed_workspace);
    }
}
