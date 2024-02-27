use anyhow::Result;
use recision::{Project, RecicionError, Workspace};

use crate::config::Config;

pub fn list(workspace: Workspace) -> Result<()> {
    let projects = workspace.get_project_names();

    if projects.is_empty() {
        println!("No projects in workspace");
        return Ok(());
    }

    let max_id_str_len = format!("{}", projects.len()).len();
    projects.iter().enumerate().for_each(|(i, project)| {
        let id_str = format!("{:>width$}", i + 1, width = max_id_str_len);
        println!("{id_str} {project}")
    });
    Ok(())
}

pub fn add(config: Config, mut workspace: Workspace, name: &str) -> Result<()> {
    if config.get_workspace().is_none() {
        return Err(RecicionError::new("no active workspace".into()).into());
    }

    println!("Adding project '{name}'");
    workspace.add_project(Project::new(name));
    workspace.write_to_file(
        config
            .get_workspace()
            .clone()
            .expect("active workspace present after early return"),
    )?;

    Ok(())
}
