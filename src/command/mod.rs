use dotenv::dotenv;

use crate::command::arg_parse::parser;
use crate::core::{migrate, run_server};

mod arg_parse;

// Define the print_usage function
// This function prints the usage instructions for the command line tool
fn print_usage() {
    println!("Usage: cargo run --release [runserver|migrate]");
}

// Define the execute_command function
// This function is an asynchronous function that executes the command specified by the user
pub async fn execute_command() {
    // Load environment variables from the .env file
    dotenv().ok();

    // Parse the command line arguments
    if let Some(args) = parser() {
        // Get the command option from the arguments
        let command_option = args.command_option();

        // Match the command option to one of the known commands and execute the corresponding function
        match command_option.as_str() {
            "runserver" => run_server().await,  // If the command is "runserver", run the server
            "migrate" => migrate().await,  // If the command is "migrate", run the migration
            _ => print_usage(),  // If the command is anything else, print the usage instructions
        }
    } else {
        // If no command line arguments were provided, print the usage instructions
        print_usage()
    }
}
