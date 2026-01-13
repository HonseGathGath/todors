#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use crate::hierarchy::Priority;

#[derive(Debug)]
enum Flag {
    Project,
    Description,
    Priority,
    Other,
}

impl Flag {
    fn classify_flag(flag: &str) -> Self {
        match flag {
            "-p" | "--project" => Flag::Project,
            "-d" | "--description" => Flag::Description,
            "--priority" => Flag::Priority,
            _ => Flag::Other,
        }
    }
}

#[derive(Debug)]
pub struct Parameters {
    tasks: Vec<String>,
    project: Option<String>,
    description: Option<String>,
    priority: Option<Priority>,
}
#[derive(Debug)]
pub struct Command {
    op: String,
    parameters: Parameters,
}
impl Parameters {
    pub fn tasks(&self) -> &Vec<String> {
        &self.tasks
    }

    pub fn fields(&self) -> (&Option<String>, &Option<String>, &Option<Priority>) {
        (&self.project, &self.description, &self.priority)
    }

    fn new() -> Self {
        Parameters {
            tasks: Vec::new(),
            project: None,
            description: None,
            priority: None,
        }
    }
}

impl Command {
    pub fn op(&self) -> &str {
        &self.op
    }
    pub fn parameters(&self) -> &Parameters {
        &self.parameters
    }
    pub fn new(args: Vec<String>) -> Self {
        let op: String = args[1].clone();
        let mut parameters: Parameters = Parameters::new();
        let mut it = args.into_iter().skip(2).peekable();

        while let Some(arg) = it.next() {
            match Flag::classify_flag(&arg) {
                Flag::Project => {
                    if let Some(value) = it.next() {
                        parameters.project = Some(value);
                    }
                }
                Flag::Priority => {
                    if let Some(value) = it.next() {
                        parameters.priority = Some(Priority::translate_priority(&value));
                    }
                }
                Flag::Description => {
                    if let Some(value) = it.next() {
                        parameters.description = Some(value);
                    }
                }
                Flag::Other => {
                    parameters.tasks.push(arg);
                }
            }
        }

        Command { op, parameters }
    }

    pub fn get_tasks(&self) -> Vec<String> {
        self.parameters.tasks.clone()
    }
}
