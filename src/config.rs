use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

use anyhow::Result;
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
}
