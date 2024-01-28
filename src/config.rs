use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

use anyhow::Result;
use dirs::config_dir;

#[derive(Debug)]
pub struct ConfigDirError {}

impl Display for ConfigDirError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "can't locate config directory")
    }
}

impl Error for ConfigDirError {}

#[allow(unused)]
#[cfg(not(test))]
fn get_config_dir() -> Option<PathBuf> {
    return config_dir()
}

#[cfg(test)]
fn get_config_dir() -> Option<PathBuf> {
    return config_dir()
}

fn get_config_file() -> Result<PathBuf> {
    match get_config_dir() {
        Some(path) => return Ok(path),
        None => Err((ConfigDirError {}).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_file() {
        let _ = get_config_file();
    }
}
