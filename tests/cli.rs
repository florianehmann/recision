use anyhow::Result;
use assert_cmd::Command;

fn build_command() -> Result<Command> {
    let cmd = Command::cargo_bin::<String>(env!("CARGO_PKG_NAME").into())?;
    Ok(cmd)
}

#[test]
#[ignore]
fn test_workspace() -> Result<()> {
    let mut cmd = build_command()?;

    cmd.arg("workspace");
    cmd.assert().success();

    Ok(())
}

#[test]
#[ignore]
fn test_workspace_deactivate() -> Result<()> {
    let mut cmd = build_command()?;

    cmd.arg("workspace").arg("deactivate");
    cmd.assert().success();

    Ok(())
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
