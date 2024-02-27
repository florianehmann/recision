use std::fs;

use anyhow::{Ok, Result};
use common::{build_command, create_test_config_dir, TEST_CONFIG_DIR};
use mockall::PredicateBooleanExt;
use predicates::str::contains;
use temp_env::with_vars;
use tempfile::NamedTempFile;

mod common;

#[test]
fn test_add_project() -> Result<()> {
    let temp_dir = create_test_config_dir()?;
    let temp_dir_path = temp_dir.path();
    let workspace_file = NamedTempFile::new()?;
    fs::remove_file(workspace_file.path())?;
    with_vars(
        [(
            TEST_CONFIG_DIR,
            Some(temp_dir_path.as_os_str().to_str().unwrap()),
        )],
        || {
            build_command()?
                .arg("workspace")
                .arg("new")
                .arg(workspace_file.path())
                .assert()
                .success();

            build_command()?
                .arg("project")
                .arg("add")
                .arg("project 1")
                .assert()
                .success();

            build_command()?
                .arg("project")
                .arg("add")
                .arg("project 2")
                .assert()
                .success();

            build_command()?
                .arg("project")
                .arg("list")
                .assert()
                .stdout(contains("project 1").and(contains("project 2")));

            Ok(())
        },
    )?;

    Ok(())
}

#[test]
fn test_add_project_twice() -> Result<()> {
    let temp_dir = create_test_config_dir()?;
    let temp_dir_path = temp_dir.path();
    let workspace_file = NamedTempFile::new()?;
    fs::remove_file(workspace_file.path())?;
    with_vars(
        [(
            TEST_CONFIG_DIR,
            Some(temp_dir_path.as_os_str().to_str().unwrap()),
        )],
        || {
            build_command()?
                .arg("workspace")
                .arg("new")
                .arg(workspace_file.path())
                .assert()
                .success();

            build_command()?
                .arg("project")
                .arg("add")
                .arg("project 1")
                .assert()
                .success();

            build_command()?
                .arg("project")
                .arg("add")
                .arg("project 1")
                .assert()
                .failure();

            build_command()?
                .arg("project")
                .arg("list")
                .assert()
                .stdout(contains("project 1"))
                .stdout(contains("project 2").not());

            Ok(())
        },
    )?;

    Ok(())
}

#[test]
fn test_add_project_invalid_name() -> Result<()> {
    let temp_dir = create_test_config_dir()?;
    let temp_dir_path = temp_dir.path();
    let workspace_file = NamedTempFile::new()?;
    fs::remove_file(workspace_file.path())?;
    with_vars(
        [(
            TEST_CONFIG_DIR,
            Some(temp_dir_path.as_os_str().to_str().unwrap()),
        )],
        || {
            build_command()?
                .arg("workspace")
                .arg("new")
                .arg(workspace_file.path())
                .assert()
                .success();

            build_command()?
                .arg("project")
                .arg("add")
                .arg("2")
                .assert()
                .failure();

            Ok(())
        },
    )?;

    Ok(())
}
