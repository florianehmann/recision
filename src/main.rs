use anyhow::Result;
use clap::ArgMatches;
use human_panic::setup_panic;

mod cli;
mod subcommands;

use cli::build_cli;
use subcommands::{run_project, run_workspace};

fn main() -> Result<()> {
    setup_panic!(Metadata {
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        authors: env!("CARGO_PKG_AUTHORS").into(),
        homepage: env!("CARGO_PKG_REPOSITORY").into(),
    });

    let matches = build_cli().get_matches();

    run(&matches)?;

    Ok(())
}

/// Runs the appropriate subcommand based on the command-line arguments.
fn run(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("workspace", submatches)) => run_workspace(&submatches)?,
        Some(("project", submatches)) => run_project(&submatches)?,
        _ => todo!("valid command isn't handled"),
    }

    Ok(())
}
