#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::{env, process::exit};
use todo::{app_state::AppState, command::Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    let command: Command = Command::new(args);
    let mut app_state = AppState::load();
    // println!("{:#?}", app_state);
    match command.op() {
        "add" => {
            if let Err(e) = app_state.handle_add(&command) {
                eprintln!("error: {e}");
                exit(1);
            }
        }
        "list" | "ls" => app_state.handle_list(0),
        _ => {
            println!("jack shi");
            exit(1);
        }
    }
    // println!("{:#?}", app_state);
}
