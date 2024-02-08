pub struct Workspace {
    projects: Vec<Project>,
}

impl Workspace {
    pub fn new() -> Self {
        return Self{ projects: Vec::new() }
    }

    pub fn add_project(&mut self, project: Project) -> &mut Self {
        self.projects.push(project);
        return self;
    }
}

pub struct Project {
    name: String,
}

impl Project {
    pub fn new(name: &str) -> Self {
        return Self{ name: String::from(name) }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    
    fn build_test_workspace() -> Workspace {
        let mut workspace = Workspace::new();

        workspace.add_project(Project::new("Project 1"))
            .add_project(Project::new("Project 2"))
            .add_project(Project::new("Project 3"));

        return workspace;
    }

    #[test]
    fn test() {
        let _ = build_test_workspace();
    }
}