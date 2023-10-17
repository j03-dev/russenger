use std::env;
use std::str::FromStr;

use dotenv::dotenv;
use rocket::{catchers, routes};
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};

use crate::core::{page_not_found, server_panic, webhook_core, webhook_verify};
use crate::core::app_state::AppState;
use crate::models::User;

#[derive(Clone)]
struct Args {
    option: String,
    _value: String,
}

impl Args {
    fn new(option: &str, value: &str) -> Self {
        Self {
            option: option.into(),
            _value: value.into(),
        }
    }

    pub fn get_option(&self) -> String {
        self.option.clone()
    }
}

fn args_parser() -> Option<Args> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => Some(Args::new(&args[1], &args[2])),
        _ => None,
    }
}

async fn run_server() {
    dotenv().ok();
    let allowed_origins = AllowedOrigins::all();
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

    rocket::build()
        .attach(cors)
        .manage(AppState::init().await)
        .mount("/", routes![webhook_verify, webhook_core])
        .register("/", catchers![page_not_found, server_panic])
        .launch()
        .await
        .unwrap();
}

async fn migrate() {
    dotenv().ok();
    let status = User::new().await.migrate().await;
    if status {
        println!("Migrate Success");
    } else {
        println!("Migrate Failed");
    }
}

fn help() {
    println!("cargo run --release runserver or migrate")
}

pub async fn command() {
    if let Some(args) = args_parser() {
        let option = args.get_option();
        match option.as_str() {
            "--runserver" => run_server().await,
            "--migrate" => migrate().await,
            _ => help()
        }
    }
}