use anyhow::Result;

mod common;

use common::build_command;

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
