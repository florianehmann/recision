use clap::{arg, Command};

pub fn build_cli() -> Command {
    Command::new("recision")
        .about("Multi-Project Decision Helper")
        .disable_help_subcommand(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("workspace")
                .about("Manage workspaces")
                .subcommand_required(false)
                .subcommand(
                    Command::new("activate")
                        .alias("a")
                        .about("Opens a workspace")
                        .arg(arg!(<DIR> "The workspace directory"))
                )
                .subcommand(
                    Command::new("new")
                        .about("Creates a new workspace")
                        .alias("n")
                        .arg(arg!(<DIR> "The workspace directory"))
                )
                .subcommand(
                    Command::new("deactivate")
                        .about("Deactivates a workspace")
                        .alias("d")
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
                        .aliases(["l", "ls"])
                )
                .subcommand(
                    Command::new("add")
                        .about("Add a project to the pool")
                        .alias("a")
                        .arg(arg!(<PROJECT_NAME> "Name of the new project"))
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove a project from the pool")
                        .aliases(["r", "rm"])
                        .arg(arg!(<PROJECT> ... "Names or IDs of the projects"))
                )
                .subcommand(
                    Command::new("reorder")
                        .about("Reorder projects in the pool by changing project IDs")
                        .after_help("If the new order is incomplete, the specified projects are moved to the top of the order.")
                        .arg(arg!(<PROJECT> ... "Names or IDs of the projects"))
                ),
        )
        .subcommand(
            Command::new("criterion")
                .about("Manages project criteria")
                .alias("c")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("list")
                        .about("List the criteria")
                        .alias("ls")
                )
                .subcommand(
                    Command::new("add")
                        .about("Add a new criterion")
                        .alias("a")
                        .arg(arg!(<NAME> "Name of the new criterion"))
                        .arg(arg!([PRIORITY] "Priority of the new criterion"))
                        .after_help(concat!(
                            "If the priority is not specified, it is set to 1.\n",
                            "Criterion names must contain at least one alphabetic character."
                        ))
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove a criterion")
                        .aliases(["r", "rm"])
                        .arg(arg!(<CRITERION> ... "Names or IDs of the criteria"))
                )
                .subcommand(
                    Command::new("reorder")
                        .about("Reorder the criteria by changing their IDs")
                        .after_help("If the new order is incomplete, the specified criteria are moved to the top of the order.")
                        .arg(arg!(<CRITERIION> ... "Names or IDs of the criteria"))
                )
                .subcommand(
                    Command::new("update-priority")
                        .about("Update the priority of a criterion")
                        .alias("up")
                        .arg(arg!(<CRITERIION> "Name or ID of the criterion"))
                        .arg(arg!(<PRIORITY> "New priority of the criterion"))
                )
        )
        .subcommand(
            Command::new("priority-set")
                .about("Manage prioritiy sets")
                .alias("ps")
                .subcommand_required(true)
                .subcommand(
                    Command::new("list")
                        .about("List priority sets")
                        .aliases(["ls", "l"])
                )
                .subcommand(
                    Command::new("add")
                        .about("Add a priority set")
                        .alias("a")
                        .arg(arg!(<NAME> "Name of the new priority set"))
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove a priority set")
                        .aliases(["rm", "r"])
                        .arg(arg!([PS] ... "Names of IDs of the priority sets"))
                )
                .subcommand(
                    Command::new("reorder")
                        .about("Reorder priority sets by changing their IDs")
                        .after_help("If the new order is incomplete, the specified priority sets are moved to the top of the order.")
                        .arg(arg!(<PS> ... "Names or IDs of the priority sets"))
                )
                .subcommand(
                    Command::new("activate")
                        .about("Activate a priority set")
                        .arg(arg!(<PS> "Name or ID of the priority set"))
                )
        )
        .subcommand(
            Command::new("weight")
                .about("Manage grid weights")
                .alias("w")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("get")
                        .about("Get the current weight of the given project and criterion")
                        .alias("g")
                        .arg(arg!(<PROJECT> "Name or ID of the project"))
                        .arg(arg!(<CRITERION> "Name or ID of the criterion"))
                )
                .subcommand(
                    Command::new("set")
                        .about("Set the current weight of the given project and criterion")
                        .alias("s")
                        .arg(arg!(<PROJECT> "Name or ID of the project"))
                        .arg(arg!(<CRITERION> "Name or ID of the criterion"))
                        .arg(arg!(<WEIGHT> "Value of the weight"))
                )
        )
        .subcommand(
            Command::new("display")
                .about("Display the grid of projects, criteria and weights")
                .alias("d")
        )
}
