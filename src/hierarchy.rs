use chrono::{DateTime, NaiveDate, Utc};

use crate::command::Command;

pub fn task_from_command(
    command: &Command,
    id: usize,
    project_id: usize,
) -> Result<Task, &'static str> {
    let name = command
        .parameters()
        .tasks()
        .get(0)
        .cloned()
        .ok_or("missing task name");

    let (_, description, priority) = command.parameters().fields();

    Ok(Task {
        id: id,
        project_id: project_id,
        name: name?,
        description: description.clone().unwrap_or_default(),
        priority: priority.clone().unwrap_or_default(),
        created_at: Utc::now(),
        due_time: None,
        completed_at: None,
    })
}

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub id: usize,
    pub parent_id: usize,
    pub tasks: Vec<Task>,
}

#[derive(Debug)]
pub struct Task {
    name: String,
    priority: Priority,
    project_id: usize,
    id: usize,
    description: String,
    created_at: DateTime<Utc>,
    due_time: Option<NaiveDate>,
    completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Priority {
    None,
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn translate_priority(priority: &str) -> Self {
        match priority {
            "low" | "l" => Priority::Low,
            "Medium" | "m" => Priority::Medium,
            "High" | "H" => Priority::High,
            _ => Priority::None,
        }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::None
    }
}
