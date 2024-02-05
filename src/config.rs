use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, remove_file, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;
use dirs::config_dir;
use mockall::automock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    active_workspace: Option<PathBuf>,
    active_priority_set: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        return Config {
            active_workspace: None,
            active_priority_set: None,
        }
    }
}

impl Config {
    fn write_to_file(&self, path: PathBuf) -> Result<()> {

        if path.exists() {
            remove_file(path.clone())?;
        }

        let toml_string = toml::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write_all(toml_string.as_bytes())?;

        return Ok(())
    }

    fn read_from_file(path: PathBuf) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut toml_string = String::new();
        file.read_to_string(&mut toml_string)?;
        let config = toml::from_str(toml_string.as_str())?;

        return Ok(config)
    }
}

#[derive(Debug)]
pub struct ConfigError {
    message: String,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ConfigError: {}", self.message)
    }
}

impl Error for ConfigError {}

#[automock]
trait ConfigDirProvider {
    fn get_config_dir(&self) -> Option<PathBuf>;
}

struct DefaultConfigDirProvider;

impl ConfigDirProvider for DefaultConfigDirProvider {
    fn get_config_dir(&self) -> Option<PathBuf> {
        return config_dir();
    }
}

fn get_config_dir(provider: &dyn ConfigDirProvider) -> Result<PathBuf> {
    return provider.get_config_dir().ok_or(
        (ConfigError {
            message: "can't locate config directory".to_string(),
        })
        .into(),
    );
}

fn get_config_file_path(config_dir_provider: &dyn ConfigDirProvider) -> Result<PathBuf> {
    let dir = get_config_dir(config_dir_provider)?
        .join("recision")
        .join("config.toml");

    return Ok(dir);
}

fn get_configuration(config_dir_provider: &dyn ConfigDirProvider) -> Result<Config> {
    let path = get_config_file_path(config_dir_provider)?;

    if !path.exists() {
        fs::create_dir_all(path.parent().expect("config dir is not root"))?;
        let default_config = Config::default();
        default_config.write_to_file(path.clone())?;
    }

    let config = Config::read_from_file(path)?;

    return Ok(config);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_config_dir_provider() -> impl ConfigDirProvider {
        let mut test_config_dir_provider = MockConfigDirProvider::new();
        test_config_dir_provider
            .expect_get_config_dir()
            .return_const(std::env::temp_dir());

        let config_file = get_config_file_path(&test_config_dir_provider)
            .expect("temp dir should exist");
        if config_file.exists() {
            fs::remove_file(config_file)
                .expect("temp config file should be deletable");
        }

        return test_config_dir_provider;
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
