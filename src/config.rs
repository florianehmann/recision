use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, remove_file, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use dirs::config_dir;
use mockall::automock;
use serde::{Deserialize, Serialize};

use crate::test_utils;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    active_workspace: Option<PathBuf>,
    active_priority_set: Option<usize>,
}

impl Config {
    fn write_to_file(&self, path: PathBuf) -> Result<()> {
        if path.exists() {
            remove_file(path.clone())?;
        }

        let toml_string = toml::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write_all(toml_string.as_bytes())?;

        Ok(())
    }

    fn read_from_file(path: PathBuf) -> Result<Self> {
        let mut file = File::open(path.clone())?;
        let mut toml_string = String::new();
        file.read_to_string(&mut toml_string)?;
        let config = toml::from_str(toml_string.as_str()).with_context(|| {
            format!(
                "Unable to parse the contents of the configuration file '{}'",
                path.to_str().unwrap()
            )
        })?;

        Ok(config)
    }

    pub fn get_workspace(&self) -> &Option<PathBuf> {
        &self.active_workspace
    }

    pub fn set_workspace(&mut self, path: Option<PathBuf>) -> Result<()> {
        if path.as_ref().is_some_and(|path| !path.exists()) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "workspace directory '{}' does not exist",
                    path.unwrap().to_str().unwrap()
                ),
            )
            .into());
        }

        self.active_workspace = path;
        self.write_to_file(get_config_file_path(&DefaultConfigDirProvider {})?)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ConfigError {
    message: String,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ConfigError {}

#[automock]
pub trait ConfigDirProvider {
    fn get_config_dir(&self) -> Option<PathBuf>;
}

pub struct DefaultConfigDirProvider;

impl ConfigDirProvider for DefaultConfigDirProvider {
    fn get_config_dir(&self) -> Option<PathBuf> {
        if test_utils::test_config_dir_is_set() {
            return Some(
                test_utils::config_dir()
                    .unwrap_or_else(|_| panic!("{} not set", test_utils::TEST_CONFIG_DIR)),
            );
        }
        config_dir()
    }
}

fn get_config_dir(provider: &dyn ConfigDirProvider) -> Result<PathBuf> {
    provider.get_config_dir().ok_or(
        (ConfigError {
            message: "can't locate config directory".to_string(),
        })
        .into(),
    )
}

fn get_config_file_path(config_dir_provider: &dyn ConfigDirProvider) -> Result<PathBuf> {
    let dir = get_config_dir(config_dir_provider)?
        .join("recision")
        .join("config.toml");

    Ok(dir)
}

pub fn get_configuration(config_dir_provider: &dyn ConfigDirProvider) -> Result<Config> {
    let path = get_config_file_path(config_dir_provider)?;

    if !path.exists() {
        fs::create_dir_all(path.parent().expect("config dir is not root"))?;
        let default_config = Config::default();
        default_config
            .write_to_file(path.clone())
            .context("error while creating new config file")?;
    }

    let config = Config::read_from_file(path).context("error while reading config file")?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_config_dir_provider() -> impl ConfigDirProvider {
        let mut test_config_dir_provider = MockConfigDirProvider::new();
        test_config_dir_provider
            .expect_get_config_dir()
            .return_const(std::env::temp_dir());

        test_config_dir_provider
    }

    #[test]
    fn test_get_real_config_dir() {
        get_config_dir(&DefaultConfigDirProvider {}).unwrap();
    }

    #[test]
    fn test_get_mock_config_dir() {
        let provider = get_test_config_dir_provider();
        get_config_dir(&provider).unwrap();
    }

    #[test]
    fn test_fail_get_config_dir() {
        let mut provider = MockConfigDirProvider::new();
        provider.expect_get_config_dir().return_const(None);

        let result = get_config_dir(&provider);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_config_file_path() {
        let provider = get_test_config_dir_provider();
        let path = get_config_file_path(&provider);
        assert!(path.is_ok());
    }

    #[test]
    fn test_get_configuration() {
        let provider = get_test_config_dir_provider();
        let result = get_configuration(&provider);
        assert!(result.is_ok());
    }
}
