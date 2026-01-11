#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::env;
use todo::command::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_refs: Vec<&str> = args.iter().map(|arg| arg.as_str()).collect();

    let command: Command = Command::new(args_refs);
    let tasks: Vec<&str> = command.get_tasks();
    for task in tasks {
        println!("{}\n", task);
    }
}
