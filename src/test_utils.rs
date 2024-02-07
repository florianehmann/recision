use std::{env, path::PathBuf};

use anyhow::Result;

pub const TEST_CONFIG_DIR: &str = "RECISION_TEST_CONFIG_DIR";

pub fn test_config_dir_is_set() -> bool {
    return !env::var(TEST_CONFIG_DIR).is_err()
}

pub fn config_dir() -> Result<PathBuf> {
    return Ok(PathBuf::from(env::var(TEST_CONFIG_DIR)?));
}
