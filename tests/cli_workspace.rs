use anyhow::{Ok, Result};

mod common;

use common::build_command;

#[test]
fn test_workspace_status() -> Result<()> {
    let mut cmd = build_command()?;

    cmd.arg("workspace");
    cmd.assert().success();

    Ok(())
}

#[test]
fn test_workspace_deactivate() -> Result<()> {
    let mut cmd = build_command()?;

    cmd.arg("workspace").arg("deactivate");
    cmd.assert().success();

    Ok(())
}