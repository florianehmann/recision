use clap::{arg, Command};
use human_panic::setup_panic;

fn cli() -> Command {
    Command::new("recision")
        .about("Multi-Project Decision Helper")
        .disable_help_subcommand(true)
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
                        .arg(arg!(<DIR> "The workspace directory")),
                )
                .subcommand(
                    Command::new("new")
                        .about("Creates a new workspace")
                        .alias("n")
                        .arg(arg!(<DIR> "The workspace directory")),
                )
                .subcommand(
                    Command::new("deactivate")
                        .about("Deactivates a workspace")
                        .alias("d"),
                ),
        )
        .subcommand(
            Command::new("project")
                .about("Manage the project pool")
                .alias("p")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("list")
                        .about("List the projects in the pool")
                        .aliases(["l", "ls"]),
                )
                .subcommand(
                    Command::new("add")
                        .about("Add a project to the pool")
                        .alias("a")
                        .arg(arg!(<PROJECT_NAME> "Name of the new project")),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove a project from the pool")
                        .aliases(["r", "rm"])
                        .arg(arg!(<PROJECT> ... "Names or IDs of the projects")),
                )
                .subcommand(
                    Command::new("reorder")
                        .about("Reorder projects in the pool by changing project IDs")
                        .after_help("If the new order is incomplete, the specified projects are moved to the top of the order.")
                        .arg(arg!(<PROJECT> ... "Names or IDs of the projects"))
                ),
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
        Some(("workspace", submatches)) => match submatches.subcommand() {
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
            }
        },
        Some(("project", submatches)) => match submatches.subcommand() {
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
                println!("Removing projects {:?}", projects)
            }
            Some(("reorder", argmatches)) => {
                let projects: Vec<_> = argmatches
                    .get_many::<String>("PROJECT")
                    .expect("required")
                    .cloned()
                    .collect();
                println!("Reordering projects {:?}", projects)
            }
            _ => unreachable!("no default behavior for project subcommand"),
        },
        _ => unreachable!("valid command isn't handled"),
    }
}
