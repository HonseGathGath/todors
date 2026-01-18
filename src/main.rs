#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::{env, process::exit};
use todo::{app_state::AppState, command::Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        exit(0);
    }

    let command: Command = Command::new(args);
    let mut app_state = AppState::load();
    
    match command.op() {
        "add" => {
            if let Err(e) = app_state.handle_add(&command) {
                eprintln!("error: {e}");
                exit(1);
            }
        }
        "list" | "ls" => app_state.handle_list(0),
        "remove" | "rm" => {
            if let Err(e) = app_state.handle_remove(&command) {
                eprintln!("error: {e}");
                exit(1);
            }
        }
        "modify" | "mod" => {
            if let Err(e) = app_state.handle_modify(&command) {
                eprintln!("error: {e}");
                exit(1);
            }
        }
        "project" => {
            if let Err(e) = app_state.handle_create_project(&command) {
                eprintln!("error: {e}");
                exit(1);
            }
        }
        "remove-project" | "rmp" => {
            if let Err(e) = app_state.handle_remove_project(&command) {
                eprintln!("error: {e}");
                exit(1);
            }
        }
        "show" => {
            if let Err(e) = app_state.handle_show(&command) {
                eprintln!("error: {e}");
                exit(1);
            }
        }
        "complete" | "done" => {
            if let Err(e) = app_state.handle_complete(&command) {
                eprintln!("error: {e}");
                exit(1);
            }
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            eprintln!("Unknown command: {}", command.op());
            eprintln!("Run 'todo help' for usage information");
            exit(1);
        }
    }
}

fn print_help() {
    println!("Todo - A simple task management CLI");
    println!();
    println!("USAGE:");
    println!("    todo <COMMAND> [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("    add <task>              Add a new task");
    println!("    list, ls                List all tasks and projects");
    println!("    remove, rm <id>         Remove a task by ID");
    println!("    modify, mod <id>        Modify a task by ID");
    println!("    show <id>               Show details of a task");
    println!("    complete, done <id>     Mark a task as complete");
    println!("    project <name>          Create a new project");
    println!("    remove-project, rmp <name>  Remove a project");
    println!("    help                    Show this help message");
    println!();
    println!("OPTIONS:");
    println!("    -p, --project <name>    Specify project name");
    println!("    -d, --description <text> Add description");
    println!("    --priority <level>      Set priority (low/l, medium/m, high/h)");
    println!("    -f, --force             Force operation (e.g., remove project with tasks)");
    println!();
    println!("EXAMPLES:");
    println!("    todo add \"Buy groceries\" -p Home --priority high");
    println!("    todo list");
    println!("    todo show 0");
    println!("    todo modify 0 \"Buy groceries and cook\" -d \"Updated task\"");
    println!("    todo complete 0");
    println!("    todo remove 0");
    println!("    todo project Work");
    println!("    todo remove-project Work --force");
}
