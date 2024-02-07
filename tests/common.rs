use std::env;

use anyhow::Result;
use assert_cmd::Command;

const TEST_FLAG: &str = "RECISION_INTEGRATION_TEST";

pub fn build_command() -> Result<Command> {
    set_test_flag();
    let cmd = Command::cargo_bin::<String>(env!("CARGO_PKG_NAME").into())?;
    Ok(cmd)
}

#[allow(dead_code)]
pub fn set_test_flag() {
    env::set_var(TEST_FLAG, "1");
}

#[allow(dead_code)]
pub fn test_flag_is_set() -> bool {
    if env::var(TEST_FLAG).is_err() {
        return false
    }

    if env::var(TEST_FLAG).unwrap().trim() != "1" {
        return false
    }

    return true
}
