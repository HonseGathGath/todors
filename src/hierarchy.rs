use std::usize;

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

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub id: usize,
    pub parent_id: usize,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
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
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

impl Priority {
    pub fn translate_priority(priority: &str) -> Self {
        match priority.to_lowercase().as_str() {
            "low" | "l" => Priority::Low,
            "medium" | "m" => Priority::Medium,
            "high" | "h" => Priority::High,
            _ => Priority::None,
        }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::None
    }
}

impl Task {
    pub fn new(
        id: usize,
        project_id: usize,
        name: String,
        description: String,
        priority: Priority,
        created_at: DateTime<Utc>,
        due_time: Option<NaiveDate>,
        completed_at: Option<DateTime<Utc>>,
    ) -> Self {
        Task {
            id,
            project_id,
            name,
            description,
            priority,
            created_at,
            due_time,
            completed_at,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn project_id(&self) -> usize {
        self.project_id
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn description(&self) -> &String {
        &self.description
    }
    pub fn priority(&self) -> Priority {
        self.priority
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    pub fn due_time(&self) -> Option<NaiveDate> {
        self.due_time
    }
    pub fn completed_at(&self) -> Option<DateTime<Utc>> {
        self.completed_at
    }
}
