use std::env;
use std::str::FromStr;

use dotenv::dotenv;
use rocket::{catchers, routes};
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};

use crate::core::{page_not_found, server_panic, webhook_core, webhook_verify};
use crate::core::app_state::AppState;
use crate::models::User;

#[derive(Debug, Clone)]
struct Args {
    _prog: String,
    option: String,
}

impl Args {
    fn new(prog: &str, option: &str) -> Self {
        Self {
            _prog: prog.into(),
            option: option.into(),
        }
    }

    pub fn get_option(&self) -> String {
        self.option.clone()
    }
}

fn args_parser() -> Option<Args> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => Some(Args::new(&args[0], &args[1])),
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
    let user_conn = User::new().await;
    println!("Connexion Success");
    let status = user_conn.migrate().await;
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
            "runserver" => run_server().await,
            "migrate" => migrate().await,
            _ => help()
        }
    }
}