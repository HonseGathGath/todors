use std::io;

use crate::{
    command::Command,
    hierarchy::{Project, Task, task_from_command},
};
use std::io::Write;

fn prompt(question: &str) -> io::Result<bool> {
    print!("{question} [y/N] ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let answer = input.trim().to_ascii_lowercase();
    Ok(answer == "y" || answer == "yes")
}

#[derive(Debug)]
pub struct AppState {
    next_task_id: usize,
    projects: Vec<Project>,
}

impl AppState {
    pub fn load() -> Self {
        let home: Project = Project {
            name: String::from("Home"),
            id: 0,
            parent_id: 0,
            tasks: Vec::new(),
        };
        AppState {
            next_task_id: 0,
            projects: vec![home],
        }
    }

    fn new_task_id(&mut self) -> usize {
        let id: usize = self.next_task_id;
        self.next_task_id += 1;
        id
    }
    pub fn handle_add(&mut self, cmd: &Command) -> Result<(), &'static str> {
        let (project_name, _, _) = cmd.parameters().fields();

        let project_id = if let Some(name) = project_name.clone() {
            self.resolve_project_id(name)
                .map_err(|_| "io error while prompting")?
                .ok_or("project not created")?
        } else {
            0
        };
        let id = self.new_task_id();

        let task = task_from_command(cmd, id, project_id).map_err(|_| "invalid command fields")?;

        self.add_task_to_project(project_id, task)?;
        Ok(())
    }

    fn find_project_id(&self, name: &str) -> Option<usize> {
        self.projects.iter().find(|p| p.name == name).map(|p| p.id)
    }

    fn resolve_project_id(&mut self, name: String) -> io::Result<Option<usize>> {
        if let Some(id) = self.find_project_id(&name) {
            return Ok(Some(id));
        }

        if prompt(&format!("Project '{name}' does not exist. Create it?'"))? {
            let id = self.create_project(name.to_string());
            Ok(Some(id))
        } else {
            Ok(None)
        }
    }
    fn create_project(&mut self, name: String) -> usize {
        let id = self.projects.len();
        let project = Project {
            name: name,
            id: id,
            parent_id: 0,
            tasks: Vec::new(),
        };

        self.projects.push(project);
        id
    }
    fn projects(&self) -> &Vec<Project> {
        &self.projects
    }

    fn add_task_to_project(&mut self, project_id: usize, task: Task) -> Result<(), &'static str> {
        let project = self
            .projects
            .iter_mut()
            .find(|p| p.id == project_id)
            .ok_or("project not found");

        project.unwrap().tasks.push(task);
        Ok(())
    }
}
