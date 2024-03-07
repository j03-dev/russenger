mod arg_parse;

use dotenv::dotenv;

use crate::core::{migrate, run_server};
use arg_parse::parser;


fn print_usage() {
    println!("Usage: cargo run --release [runserver|migrate]");
}

pub async fn execute_command() {
    dotenv().ok();

    if let Some(args) = parser() {
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
