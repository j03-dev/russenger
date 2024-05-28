//! The `cli` module contains the command line interface for the application.
//!
//! It includes the `command_handler` function that starts the server.
//!
//! # Functions
//!
//! * `command_handler`: This function starts the server. It first checks if the `ACTION_REGISTRY` contains the `Main` action. If not, it panics. Then it sets up CORS, reads the `PORT` and `HOST` environment variables, and starts the server with these settings.
//!
//! # Examples
//!
//! Running the server:
//!
//! ```rust
//! use russenger::cli::launch;
//!
//! #[russenger::main]
//! async fn main() {
//!     launch().await;
//! }
//! ```
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use crate::core::{
    action::ACTION_REGISTRY,
    app_state::AppState,
    services::{index, webhook_core, webhook_verify}, // core services
};
use crate::query::Query;

use std::env;

fn print_info(host: &str, port: u16) {
    let url = format!("http://{}:{}", host, port);
    println!("Endpoints:");
    println!("  GET: {}/ - Root endpoint", url);
    println!("  GET: {}/webhook - Webhook verification endpoint", url);
    println!("  POST: {}/webhook - Webhook core endpoint", url);
}

async fn run_server() {
    if !ACTION_REGISTRY.lock().await.contains_key("Main") {
        panic!("'russenger_app!' should contain `Main` action");
    }
    let app_state = AppState::init().await;
    let host = env::var("HOST").unwrap_or("0.0.0.0".into());
    let port = env::var("PORT")
        .unwrap_or("2453".into())
        .parse()
        .unwrap_or(2453);
    print_info(&host, port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(index)
            .service(webhook_verify)
            .service(webhook_core)
            .service(fs::Files::new("/static", "static").show_files_listing())
    })
    .bind((host.clone(), port))
    .expect("Failed to run this server: pls check the port if it's already used!")
    .run()
    .await
    .expect("sever is crashed");
}

async fn migrate() {
    let query = Query::new().await;
    println!("Connection successful!");
    let migration_result = match query.migrate().await {
        true => "Migration successful!",
        false => "Migration failed",
    };
    println!("{migration_result}");
}

fn print_usage() {
    println!("Usage: cargo run --release [runserver|migrate]");
}

fn parser() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => Some(args[1].clone()),
        3 => Some(args[2].clone()),
        _ => None,
    }
}

pub async fn launch() {
    dotenv().ok();
    match parser() {
        Some(option) => match option.as_str() {
            "runserver" => run_server().await,
            "migrate" => migrate().await,
            _ => print_usage(),
        },
        None => print_usage(),
    }
}
