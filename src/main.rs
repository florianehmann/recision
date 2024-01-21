use clap::{Command, arg};
use human_panic::setup_panic;


fn cli() -> Command {
    Command::new("recision")
        .about("Multi-Project Decision Helper")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("workspace")
                .about("Manage workspaces")
                .alias("w")
                .subcommand_required(false)
                .subcommand(
                    Command::new("activate")
                        .alias("a")
                        .about("Opens a workspace")
                        .arg(arg!(<DIR> "The workspace directory"))
                )
                .subcommand(
                    Command::new("new")
                        .alias("n")
                        .about("Creates a new workspace")
                        .arg(arg!(<DIR> "The workspace directory"))
                )
                .subcommand(
                    Command::new("deactivate")
                        .alias("d")
                        .about("Deactivates a workspace")
                )
        )
}

fn main() {
    setup_panic!(Metadata {
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        authors: env!("CARGO_PKG_AUTHORS").into(),
        homepage: env!("CARGO_PKG_REPOSITORY").into(),
    });

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("workspace", submatches)) => {
            match submatches.subcommand() {
                Some(("activate", argmatches)) => {
                    let dir = argmatches.get_one::<String>("DIR").expect("required");
                    println!("Activating workspace {dir}");
                    todo!();
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
                    println!("Currently active workspace:");
                    todo!();
                },
            }
        }
        _ => unreachable!("can't run recision without subcommand"),
    }
}
