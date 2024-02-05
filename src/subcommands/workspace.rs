use std::{error::Error, fmt, path::PathBuf};

use anyhow::{Ok, Result};
use path_absolutize::Absolutize;

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

pub fn activate(path: PathBuf, config: &mut Config) -> Result<()> {
    config.set_workspace(path.absolutize().unwrap().to_path_buf())?;

    return Ok(());
}

pub fn status(config: &mut Config) -> Result<()> {
    match config.get_workspace() {
        Some(path) => println!("Active workspace: {}", path.to_str().unwrap()),
        None => println!("No active workspace"),
    }

    return Ok(());
}
