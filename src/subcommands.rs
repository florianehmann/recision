use std::path::PathBuf;

use anyhow::Result;
use clap::ArgMatches;
use path_absolutize::Absolutize;
use recision::{RecicionError, Workspace};

mod project;
mod workspace;

use crate::config::{get_configuration, DefaultConfigDirProvider};

pub fn run_workspace(matches: &ArgMatches) -> Result<()> {
    let mut config = get_configuration(&DefaultConfigDirProvider {})?;
    match matches.subcommand() {
        Some(("activate", argmatches)) => {
            let file = PathBuf::from(argmatches.get_one::<String>("FILE").expect("required"));
            workspace::activate(file, &mut config)?;
        }
        Some(("new", argmatches)) => {
            let file = PathBuf::from(argmatches.get_one::<String>("FILE").expect("required"));
            workspace::new(file, &mut config)?;
        }
        Some(("deactivate", _)) => {
            workspace::deactivate(&mut config)?;
        }
        _ => {
            workspace::status(&mut config)?;
        }
    }

    Ok(())
}

pub fn run_project(matches: &ArgMatches) -> Result<()> {
    let config = get_configuration(&DefaultConfigDirProvider {})?;
    #[allow(unused_mut)] // TODO shut clippy up for now
    let mut workspace = Workspace::read_from_file(
        config
            .get_workspace()
            .clone()
            .ok_or(RecicionError::new("no active workspace".into()))?
            .absolutize()
            .unwrap()
            .to_path_buf(),
    )?;

    match matches.subcommand() {
        Some(("list", _)) => project::list(workspace),
        Some(("add", argmatches)) => {
            let project = argmatches
                .get_one::<String>("PROJECT_NAME")
                .expect("required");
            project::add(config, workspace, project)
        }
        Some(("remove", argmatches)) => {
            let projects: Vec<_> = argmatches
                .get_many::<String>("PROJECT")
                .expect("required")
                .cloned()
                .collect();
            println!("Removing projects {:?}", projects);
            todo!();
        }
        Some(("reorder", argmatches)) => {
            let projects: Vec<_> = argmatches
                .get_many::<String>("PROJECT")
                .expect("required")
                .cloned()
                .collect();
            println!("Reordering projects {:?}", projects);
            todo!();
        }
        _ => unreachable!("no default behavior for project subcommand"),
    }
}

#[allow(unused)]
pub fn run_criterion(matches: &ArgMatches) -> Result<()> {
    todo!();
}

#[allow(unused)]
pub fn run_priority_set(matches: &ArgMatches) -> Result<()> {
    todo!();
}

#[allow(unused)]
pub fn run_weight(matches: &ArgMatches) -> Result<()> {
    todo!();
}

#[allow(unused)]
pub fn run_display(matches: &ArgMatches) -> Result<()> {
    todo!();
}
