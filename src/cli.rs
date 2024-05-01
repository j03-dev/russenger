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
//! use russenger::cli::command_handler;
//!
//! #[russenger::main]
//! async fn main() {
//!     command_handler().await;
//! }
//! ```
use dotenv::dotenv;
use rocket::{catchers, fs::FileServer, routes};
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};

use std::str::FromStr;

use crate::core::{
    action::ACTION_REGISTRY,
    app_state::AppState,
    services::{page_not_found, server_panic, webhook_core, webhook_verify}, // core services
};
use crate::query::Query;

use std::env;

async fn run_server() {
    if !ACTION_REGISTRY.lock().await.contains_key("Main") {
        panic!("'russenger_app!' should containt `Main` action");
    }
    let allowed_origins = AllowedOrigins::some_regex(&["graph.facebook.com"]);

    let allowed_methods: AllowedMethods = ["Get", "Post"]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    let port: i32 = env::var("PORT")
        .unwrap_or("2453".into())
        .parse()
        .expect("Should Containt number");
    let addr = env::var("HOST").unwrap_or("0.0.0.0".into());

    let figment = rocket::Config::figment()
        .merge(("port", port))
        .merge(("address", addr));

    rocket::custom(figment)
        .attach(cors)
        .manage(AppState::init().await)
        .mount("/", routes![webhook_verify, webhook_core])
        .mount("/static", FileServer::from("static"))
        .register("/", catchers![page_not_found, server_panic])
        .launch()
        .await
        .expect("Failed to run Rocket server");
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

pub async fn command_handler() {
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
