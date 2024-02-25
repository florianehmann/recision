use anyhow::Result;
use recision::Workspace;

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
