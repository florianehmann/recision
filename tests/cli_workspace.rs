use anyhow::{Ok, Result};
use predicates::str::contains;

use common::{build_command, create_test_config_dir, TEST_CONFIG_DIR};
use temp_env::with_vars;

mod common;

#[test]
fn test_workspace_status() -> Result<()> {
    let temp_dir = create_test_config_dir()?;
    let temp_dir_path = temp_dir.path();
    with_vars(
        [(
            TEST_CONFIG_DIR,
            Some(temp_dir_path.as_os_str().to_str().unwrap()),
        )],
        || {
            build_command()?.arg("workspace").assert().success();

            Ok(())
        },
    )?;

    Ok(())
}

#[test]
fn test_workspace_activate() -> Result<()> {
    let temp_dir = create_test_config_dir()?;
    let temp_dir_path = temp_dir.path();
    with_vars(
        [(
            TEST_CONFIG_DIR,
            Some(temp_dir_path.as_os_str().to_str().unwrap()),
        )],
        || {
            build_command()?
                .arg("workspace")
                .arg("deactivate")
                .assert()
                .success();

            build_command()?
                .arg("workspace")
                .arg("activate")
                .arg(temp_dir_path)
                .assert()
                .success();

            build_command()?.arg("workspace").assert().stdout(contains(
                temp_dir_path.file_name().unwrap().to_str().unwrap(),
            ));

            Ok(())
        },
    )?;

    Ok(())
}

#[test]
fn test_workspace_deactivate() -> Result<()> {
    let temp_dir = create_test_config_dir()?;
    let temp_dir_path = temp_dir.path();
    with_vars(
        [(
            TEST_CONFIG_DIR,
            Some(temp_dir_path.as_os_str().to_str().unwrap()),
        )],
        || {
            build_command()?
                .arg("workspace")
                .arg("activate")
                .arg(temp_dir.path().as_os_str())
                .assert()
                .success();

            build_command()?
                .arg("workspace")
                .arg("deactivate")
                .assert()
                .success();

            build_command()?
                .arg("workspace")
                .assert()
                .success()
                .stdout(contains("No active workspace"));

            Ok(())
        },
    )?;

    Ok(())
}
