use std::path::PathBuf;

use anyhow::Result;
use clap::ArgMatches;

mod workspace;

use crate::config::{get_configuration, DefaultConfigDirProvider};

pub fn run_workspace(matches: &ArgMatches) -> Result<()> {
    let mut config = get_configuration(&DefaultConfigDirProvider {})?;
    match matches.subcommand() {
        Some(("activate", argmatches)) => {
            let dir = PathBuf::from(argmatches.get_one::<String>("DIR").expect("required"));
            workspace::activate(dir, &mut config)?;
        }
        Some(("new", argmatches)) => {
            let dir = argmatches.get_one::<String>("DIR").expect("required");
            println!("Creating workspace {dir}");
            todo!();
        }
        Some(("deactivate", _)) => {
            println!("Deactivating workspace");
            todo!();
        }
        _ => {
            workspace::status(&mut config)?;
        }
    }

    Ok(())
}

pub fn run_project(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("list", _)) => {
            println!("Listing projects");
            todo!();
        }
        Some(("add", argmatches)) => {
            let project = argmatches
                .get_one::<String>("PROJECT_NAME")
                .expect("required");
            println!("Adding project {project}");
            todo!();
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
