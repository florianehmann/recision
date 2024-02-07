use anyhow::Result;

mod common;

use common::{build_command, test_flag_is_set};

use crate::common::set_test_flag;

#[test]
fn test_test_flag_set() {
    set_test_flag();
    assert_eq!(true, test_flag_is_set());
}

#[test]
#[ignore]
fn test_project_ls() -> Result<()> {
    let mut cmd = build_command()?;

    cmd.arg("project").arg("ls");
    cmd.assert().success();

    Ok(())
}

#[test]
#[ignore]
fn test_criterion_ls() -> Result<()> {
    let mut cmd = build_command()?;

    cmd.arg("criterion").arg("ls");
    cmd.assert().success();

    Ok(())
}

#[test]
#[ignore]
fn test_display() -> Result<()> {
    let mut cmd = build_command()?;

    cmd.arg("display");
    cmd.assert().success();

    Ok(())
}
