#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

#[derive(Debug)]
enum FLAG {
    PROJECT,
    SHOW,
    OTHER,
}

impl FLAG {
    fn map_to_FLAG(flag: &str) -> Self {
        match flag {
            "p" => FLAG::PROJECT,
            "s" => FLAG::SHOW,
            _ => FLAG::OTHER,
        }
    }
    fn map_to_str<'a>(&self) -> &'a str {
        match self {
            FLAG::PROJECT => "p",
            FLAG::SHOW => "s",
            FLAG::OTHER => "_",
        }
    }
}

#[derive(Debug)]
struct Parameters<'a> {
    tasks: Vec<&'a str>,
    project: &'a str,
}
#[derive(Debug)]
pub struct Command<'a> {
    op: &'a str,
    flags: Vec<FLAG>,
    paramters: Parameters<'a>,
}
impl<'a> Parameters<'a> {
    fn new() -> Self {
        Parameters {
            tasks: Vec::new(),
            project: "",
        }
    }
}

impl<'a> Command<'a> {
    pub fn new(args: Vec<&'a str>) -> Self {
        let op: &str = &args[1];
        let mut flags: Vec<FLAG> = Vec::new();
        let mut parameters: Parameters = Parameters::new();
        for (index, arg) in args[2..].iter().enumerate() {
            if arg.starts_with("-") {
                let flag: FLAG = FLAG::map_to_FLAG(&arg[1..]);
                flags.push(flag);
                if &arg[1..] == "p" {
                    parameters.project = args[2..][index + 1];
                }
                continue;
            } else if arg != &parameters.project {
                parameters.tasks.push(arg);
            }
        }
        Command {
            op,
            flags: flags,
            paramters: parameters,
        }
    }
    pub fn get_tasks(&'a self) -> Vec<&'a str> {
        self.paramters.tasks.clone()
    }
}
