use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use config::Config;
use dirs::config_dir;
use mockall::automock;

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

fn get_config_file_path() -> Result<PathBuf> {
    let dir = get_config_dir(&(DefaultConfigDirProvider {}))?
        .join("recision")
        .join("config.toml");

    return Ok(dir);
}

fn get_configuration() -> Result<()> {
    let path = get_config_file_path()?;

    // TODO instead of creating a new empty file write serizalized default config
    if !path.exists() {
        fs::create_dir_all(path.parent().expect("config dir is not root"))?;
        fs::File::create(path.clone())?;
    }

    let config = Config::builder()
        .add_source(config::File::with_name(
            path.to_str().expect("path should be valid unicode"),
        ))
        .build()
        .expect("file should exist by now");

    println!("{:?}", config);

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_real_config_dir() {
        get_config_dir(&DefaultConfigDirProvider {}).unwrap();
    }

    #[test]
    fn test_get_mock_config_dir() {
        let mut provider = MockConfigDirProvider::new();
        provider
            .expect_get_config_dir()
            .return_const(PathBuf::from("/tmp/config"));

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
        let path = get_config_file_path();
        println!("{:?}", path);
    }

    #[test]
    fn test_get_configuration() {
        let _ = get_configuration();
    }
}
