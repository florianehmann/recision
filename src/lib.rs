use core::fmt;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    fs::{remove_file, File},
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct RecicionError {
    message: String,
}

impl RecicionError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for RecicionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RecicionError {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    projects: Vec<Project>,
    criteria: Vec<Criterion>,
    priority_sets: Vec<PrioritySet>,
    active_priority_set: Option<String>,
}

impl Workspace {
    pub fn new() -> Self {
        Self {
            projects: Vec::new(),
            criteria: Vec::new(),
            priority_sets: Vec::new(),
            active_priority_set: None,
        }
    }

    pub fn add_project(&mut self, project: Project) -> Result<&mut Self> {
        if self.get_project(project.get_name()).is_some() {
            return Err(RecicionError::new(format!(
                "project {} already exists",
                project.get_name()
            ))
            .into());
        }

        self.projects.push(project);
        Ok(self)
    }

    pub fn add_criterion(&mut self, criterion: Criterion) -> &mut Self {
        // TODO check if criterion by the name already exists
        self.criteria.push(criterion);
        self
    }

    pub fn get_project(&mut self, name: &str) -> Option<&mut Project> {
        self.projects
            .iter_mut()
            .find(|project| project.name == name)
    }

    pub fn get_project_names(&self) -> Vec<String> {
        self.projects.iter().map(|p| p.name.clone()).collect()
    }

    pub fn get_criterion(&mut self, name: &str) -> Option<&mut Criterion> {
        self.criteria
            .iter_mut()
            .find(|criterion| criterion.name == name)
    }

    pub fn add_priority_set(&mut self, name: &str) -> Result<&mut Self> {
        if self.get_priority_set(name).is_some() {
            return Err(RecicionError::new(format!("priority set {} already exists", name)).into());
        }

        let ps = PrioritySet::new(name);
        self.priority_sets.push(ps);

        Ok(self)
    }

    pub fn get_priority_set(&self, name: &str) -> Option<&PrioritySet> {
        self.priority_sets.iter().find(|ps| ps.name == name)
    }

    pub fn get_priority_set_mut(&mut self, name: &str) -> Option<&mut PrioritySet> {
        self.priority_sets.iter_mut().find(|ps| ps.name == name)
    }

    pub fn activate_priority_set(&mut self, name: &str) -> Result<()> {
        self.get_priority_set(name)
            .ok_or(RecicionError::new(format!("no priority set {}", name)))?;
        self.active_priority_set = Some(name.into());
        Ok(())
    }

    pub fn write_to_file(&self, path: PathBuf) -> Result<()> {
        if path.exists() {
            remove_file(path.clone()).with_context(|| "removing existing worspace file")?;
        }

        let toml_string = toml::to_string_pretty(self)?;
        let mut file = File::create(path.clone()).with_context(|| "creating new workspace file")?;
        file.write_all(toml_string.as_bytes())?;

        Ok(())
    }

    pub fn read_from_file(path: PathBuf) -> Result<Self> {
        let mut file = File::open(path.clone())
            .with_context(|| format!("opening workspace file {}", path.to_str().unwrap()))?;
        let mut toml_string = String::new();
        file.read_to_string(&mut toml_string)
            .with_context(|| "reading workspace from file")?;
        let workspace = toml::from_str(toml_string.as_str())
            .with_context(|| "parsing contents wof workspace file")?;

        Ok(workspace)
    }

    pub fn set_weight(
        &mut self,
        project_name: &str,
        criterion_name: &str,
        weight: i32,
    ) -> Result<()> {
        self.get_criterion(criterion_name)
            .ok_or(RecicionError::new(format!(
                "no criterion {}",
                criterion_name
            )))?;

        self.get_project(project_name)
            .ok_or(RecicionError::new(format!("no project {}", project_name)))?
            .weights
            .insert(criterion_name.into(), weight);

        Ok(())
    }

    pub fn get_weight(&mut self, project_name: &str, criterion_name: &str) -> Result<i32> {
        self.get_criterion(criterion_name)
            .ok_or(RecicionError::new(format!(
                "no criterion {}",
                criterion_name
            )))?;

        Ok(*self
            .get_project(project_name)
            .ok_or(RecicionError::new(format!("no project {}", project_name)))?
            .weights
            .get(criterion_name)
            .unwrap_or(&0))
    }

    pub fn set_priority(&mut self, criterion_name: &str, priority: f64) -> Result<()> {
        self.get_criterion(criterion_name)
            .ok_or(RecicionError::new(format!(
                "no criterion {}",
                criterion_name
            )))?;

        let priority_set_name = self
            .active_priority_set
            .clone()
            .ok_or(RecicionError::new("no active priority set".into()))?;
        let priority_set = self
            .get_priority_set_mut(priority_set_name.as_str())
            .expect("active priority set should be in the collections of priority sets");

        priority_set
            .priorities
            .insert(criterion_name.into(), priority);

        Ok(())
    }

    pub fn calculate_score(&self) -> Result<HashMap<String, f64>> {
        let priority_set_name = self
            .active_priority_set
            .clone()
            .ok_or(RecicionError::new("no active priority set".into()))?;
        let priority_set = self
            .get_priority_set(priority_set_name.as_str())
            .expect("active priority set should be in the collections of priority sets");

        let mut result = HashMap::new();
        self.projects.iter().for_each(|project| {
            let score = project.calculate_score(&self.criteria, priority_set);
            result.insert(project.name.clone(), score);
        });

        Ok(result)
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    name: String,
    weights: HashMap<String, i32>,
}

impl Project {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            weights: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn calculate_score(&self, criteria: &[Criterion], priority_set: &PrioritySet) -> f64 {
        let mut score = 0.0;
        criteria.iter().for_each(|criterion| {
            // TODO define default priority in the configuration
            let priority = priority_set.priorities.get(&criterion.name).unwrap_or(&1.0);
            let weight = self.weights.get(&criterion.name).unwrap_or(&0);

            score += *weight as f64 * priority;
        });

        score
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Criterion {
    name: String,
}

impl Criterion {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrioritySet {
    name: String,
    priorities: HashMap<String, f64>,
}

impl PrioritySet {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            priorities: HashMap::new(),
        }
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
            .unwrap()
            .add_project(Project::new("Project 2"))
            .unwrap()
            .add_project(Project::new("Project ="))
            .unwrap()
            .add_project(Project::new("Project [toml]"))
            .unwrap()
            .add_project(Project::new("Project\nNewline"))
            .unwrap();

        workspace
            .add_criterion(Criterion::new("Fun"))
            .add_criterion(Criterion::new("Useful"));

        workspace
            .add_priority_set("Workday")
            .unwrap()
            .add_priority_set("Weekend")
            .unwrap();

        workspace.activate_priority_set("Workday").unwrap();
        workspace.set_priority("Fun", 1.0).unwrap();
        workspace.set_priority("Useful", 2.0).unwrap();

        workspace.activate_priority_set("Weekend").unwrap();
        workspace.set_priority("Fun", 2.0).unwrap();
        workspace.set_priority("Useful", 1.0).unwrap();

        workspace
    }

    #[test]
    fn test_building_workpace() {
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

    #[test]
    fn test_set_weight() {
        let mut ws = build_test_workspace();
        ws.set_weight("Project 1", "Fun", 1).unwrap();
    }

    #[test]
    fn test_set_weight_fail_project() {
        let mut ws = build_test_workspace();
        let result = ws.set_weight("Project 11", "Fun", 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_weight_fail_criterion() {
        let mut ws = build_test_workspace();
        let result = ws.set_weight("Project 1", "Funn", 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_existing_weight() {
        let mut ws = build_test_workspace();
        let weight = 1;
        ws.set_weight("Project 1", "Fun", weight).unwrap();
        assert_eq!(weight, ws.get_weight("Project 1", "Fun").unwrap());
    }

    #[test]
    fn test_get_default_weight() {
        let mut ws = build_test_workspace();
        assert_eq!(0, ws.get_weight("Project 1", "Fun").unwrap());
    }

    #[test]
    fn test_get_weight_fail_project() {
        let mut ws = build_test_workspace();
        let result = ws.get_weight("Project 11", "Fun");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_weight_fail_criterion() {
        let mut ws = build_test_workspace();
        let result = ws.get_weight("Project 1", "Funn");
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_score() {
        let mut ws = build_test_workspace();

        ws.set_weight("Project 1", "Fun", 1).unwrap();
        ws.set_weight("Project 1", "Useful", -1).unwrap();

        ws.set_weight("Project 2", "Fun", 2).unwrap();

        ws.activate_priority_set("Workday").unwrap();

        let scores = ws.calculate_score().unwrap();

        assert_eq!(*scores.get("Project 1").unwrap(), -1.0);
        assert_eq!(*scores.get("Project 2").unwrap(), 2.0);

        ws.activate_priority_set("Weekend").unwrap();

        let scores = ws.calculate_score().unwrap();

        assert_eq!(*scores.get("Project 1").unwrap(), 1.0);
        assert_eq!(*scores.get("Project 2").unwrap(), 4.0);
    }
}
