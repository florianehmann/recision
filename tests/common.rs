use std::env;

use anyhow::Result;
use assert_cmd::Command;
use tempfile::{tempdir, TempDir};
#[allow(dead_code)]
pub const TEST_CONFIG_DIR: &str = "RECISION_TEST_CONFIG_DIR";

pub fn build_command() -> Result<Command> {
    let cmd = Command::cargo_bin::<String>(env!("CARGO_PKG_NAME").into())?;
    Ok(cmd)
}

#[allow(dead_code)]
pub fn create_test_config_dir() -> Result<TempDir> {
    Ok(tempdir()?)
}
