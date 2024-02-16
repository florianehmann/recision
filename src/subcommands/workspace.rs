use std::{error::Error, fmt, path::PathBuf};

use anyhow::{Ok, Result};
use path_absolutize::Absolutize;
use recision::{RecicionError, Workspace};

use crate::config::Config;

#[derive(Debug)]
pub struct WorkspaceError {
    message: String,
}

impl fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for WorkspaceError {}

pub fn new(path: PathBuf, config: &mut Config) -> Result<()> {
    if path.exists() {
        return Err(
            RecicionError::new(format!("file {} already exists", path.to_str().unwrap())).into(),
        );
    }

    let workspace = Workspace::new();
    workspace.write_to_file(path.clone())?;
    config.set_workspace(Some(path))?;

    Ok(())
}

pub fn activate(path: PathBuf, config: &mut Config) -> Result<()> {
    let _ = Workspace::read_from_file(path.absolutize().unwrap().to_path_buf())?;

    config.set_workspace(Some(path.absolutize().unwrap().to_path_buf()))?;

    Ok(())
}

pub fn deactivate(config: &mut Config) -> Result<()> {
    config.set_workspace(None)?;

    Ok(())
}

pub fn status(config: &mut Config) -> Result<()> {
    match config.get_workspace() {
        Some(path) => println!(
            "Active workspace: {}",
            path.absolutize().unwrap().to_str().unwrap()
        ),
        None => println!("No active workspace"),
    }

    Ok(())
}
