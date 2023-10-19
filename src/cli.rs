use crate::core::{migrate, run_server};
use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct CommandLineArgs {
    program_name: String,
    command_option: String,
}

impl CommandLineArgs {
    fn new(program_name: &str, command_option: &str) -> Self {
        Self {
            program_name: program_name.into(),
            command_option: command_option.into(),
        }
    }

    pub fn command_option(&self) -> String {
        self.command_option.clone()
    }

    pub fn command_program_name(&self) -> String {
        self.program_name.clone()
    }
}

fn parse_command_line_args() -> Option<CommandLineArgs> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => Some(CommandLineArgs::new(&args[0], &args[1])),
        3 => Some(CommandLineArgs::new(&args[0], &args[2])),
        _ => None,
    }
}

fn print_usage() {
    println!("Usage: cargo run --release [runserver|migrate]");
}

pub async fn execute_command() {
    dotenv().ok();
    if let Some(args) = parse_command_line_args() {
        let command_option = args.command_option();
        match command_option.as_str() {
            "runserver" => run_server().await,
            "migrate" => migrate().await,
            _ => print_usage(),
        }
    } else {
        print_usage()
    }
}
