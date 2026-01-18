use std::io;

use crate::{
    command::Command,
    database::Database,
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
    db: Database,
}

impl AppState {
    pub fn load() -> Self {
        let db = Database::new().expect("Failed to initialize database");
        
        let mut projects = db.load_projects().unwrap_or_else(|_| Vec::new());

        let next_task_id = db.load_next_task_id().unwrap_or(0);

        // If no projects exist, create and save the default Home project
        if projects.is_empty() {
            let home_project = Project {
                name: String::from("Home"),
                id: 0,
                parent_id: 0,
                tasks: Vec::new(),
            };
            let _ = db.save_projects(&[home_project.clone()]);
            projects = vec![home_project];
        }

        AppState {
            next_task_id,
            projects,
            db,
        }
    }

    fn save(&self) {
        if let Err(e) = self.db.save_projects(&self.projects) {
            eprintln!("Failed to save projects: {}", e);
        }
        if let Err(e) = self.db.save_next_task_id(self.next_task_id) {
            eprintln!("Failed to save next_task_id: {}", e);
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
        self.save();
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
        let id = self.projects.iter().map(|p| p.id).max().unwrap_or(0) + 1;
        let project = Project {
            name: name,
            id: id,
            parent_id: 0,
            tasks: Vec::new(),
        };

        self.projects.push(project);
        self.save();
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
    fn children_of(&self, parent_id: usize) -> impl Iterator<Item = &Project> {
        self.projects
            .iter()
            .filter(move |p| p.parent_id == parent_id && p.id != parent_id)
    }

    fn print_subtree(&self, project_id: usize, depth: usize) {
        if let Some(p) = self.projects.iter().find(|p| p.id == project_id) {
            println!("{:indent$}{}", "", p.name, indent = depth * 2);

            for task in &p.tasks {
                println!(
                    "{:indent$}- [{}]",
                    "",
                    task.name(),
                    indent = (depth + 1) * 2
                );
            }
            for child in self.children_of(p.id) {
                self.print_subtree(child.id, depth + 1);
            }
        }
    }

    pub fn handle_list(&self, project_id: usize) {
        self.print_subtree(project_id, 0);
    }
    
    pub fn handle_remove(&mut self, cmd: &Command) -> Result<(), &'static str> {
        if let Some(task_id) = cmd.parameters().task_id() {
            // Remove task by ID in a single pass
            let mut found = false;
            for project in self.projects.iter_mut() {
                let initial_len = project.tasks.len();
                project.tasks.retain(|t| t.id() != task_id);
                if project.tasks.len() < initial_len {
                    found = true;
                    break;
                }
            }
            if !found {
                return Err("task not found");
            }
        } else {
            return Err("task ID required");
        }
        self.save();
        Ok(())
    }

    pub fn handle_modify(&mut self, cmd: &Command) -> Result<(), &'static str> {
        let task_id = cmd.parameters().task_id().ok_or("task ID required")?;
        
        let mut found = false;
        'outer: for project in self.projects.iter_mut() {
            for task in project.tasks.iter_mut() {
                if task.id() == task_id {
                    let new_task = task_from_command(cmd, task_id, task.project_id())
                        .map_err(|_| "invalid command fields")?;
                    *task = new_task;
                    found = true;
                    break 'outer;
                }
            }
        }
        
        if !found {
            return Err("task not found");
        }
        
        self.save();
        Ok(())
    }

    pub fn handle_create_project(&mut self, cmd: &Command) -> Result<(), &'static str> {
        let (project_name, _, _) = cmd.parameters().fields();
        
        if let Some(name) = project_name {
            if self.find_project_id(name).is_some() {
                return Err("project already exists");
            }
            self.create_project(name.clone());
            println!("Project '{}' created", name);
            Ok(())
        } else if !cmd.parameters().tasks().is_empty() {
            let name = cmd.parameters().tasks()[0].clone();
            if self.find_project_id(&name).is_some() {
                return Err("project already exists");
            }
            self.create_project(name.clone());
            println!("Project '{}' created", name);
            Ok(())
        } else {
            Err("project name required")
        }
    }

    pub fn handle_remove_project(&mut self, cmd: &Command) -> Result<(), &'static str> {
        let (project_name, _, _) = cmd.parameters().fields();
        let force = cmd.parameters().force();
        
        let name = if let Some(n) = project_name {
            n.clone()
        } else if !cmd.parameters().tasks().is_empty() {
            cmd.parameters().tasks()[0].clone()
        } else {
            return Err("project name required");
        };
        
        let project_id = self.find_project_id(&name).ok_or("project not found")?;
        
        // Don't allow removing Home project
        if project_id == 0 {
            return Err("cannot remove Home project");
        }
        
        // Check if project has tasks
        if let Some(project) = self.projects.iter().find(|p| p.id == project_id) {
            if !project.tasks.is_empty() && !force {
                return Err("project has tasks, use --force to remove anyway");
            }
        }
        
        self.projects.retain(|p| p.id != project_id);
        self.save();
        println!("Project '{}' removed", name);
        Ok(())
    }

    pub fn handle_show(&self, cmd: &Command) -> Result<(), &'static str> {
        let task_id = cmd.parameters().task_id().ok_or("task ID required")?;
        
        for project in &self.projects {
            for task in &project.tasks {
                if task.id() == task_id {
                    println!("Task ID: {}", task.id());
                    println!("Name: {}", task.name());
                    println!("Project: {}", project.name);
                    println!("Description: {}", task.description());
                    println!("Priority: {:?}", task.priority());
                    println!("Created: {}", task.created_at().format("%Y-%m-%d %H:%M:%S"));
                    if let Some(due) = task.due_time() {
                        println!("Due: {}", due);
                    }
                    if let Some(completed) = task.completed_at() {
                        println!("Completed: {}", completed.format("%Y-%m-%d %H:%M:%S"));
                    }
                    return Ok(());
                }
            }
        }
        
        Err("task not found")
    }

    pub fn handle_complete(&mut self, cmd: &Command) -> Result<(), &'static str> {
        let task_id = cmd.parameters().task_id().ok_or("task ID required")?;
        
        let mut found = false;
        for project in self.projects.iter_mut() {
            for task in project.tasks.iter_mut() {
                if task.id() == task_id {
                    task.mark_complete();
                    found = true;
                    println!("Task {} marked as complete", task_id);
                    break;
                }
            }
            if found {
                break;
            }
        }
        
        if !found {
            return Err("task not found");
        }
        
        self.save();
        Ok(())
    }
    
    pub fn handle_remove_task(&mut self, id: usize, project_id: usize) {
        self.projects.iter_mut().for_each(|p| {
            if p.id == project_id {
                p.tasks.retain(|t| id != t.id());
            }
        });
        self.save();
    }
    pub fn handle_modify_task(&mut self, id: usize, command: &Command) {
        'outer: for project in self.projects.iter_mut() {
            for task in project.tasks.iter_mut() {
                if task.id() == id {
                    let new_task = task_from_command(command, id, task.project_id());
                    *task = new_task.ok().unwrap();
                    break 'outer;
                }
            }
        }
        self.save();
    }
}
