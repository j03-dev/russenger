#[macro_use]
extern crate rocket;

use app::core;
use potato::hooks::hooks_verify;
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins};
use std::{error::Error, str::FromStr};

pub mod app;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let allowed_origins = AllowedOrigins::some_exact(&["facebook.com"]);
    let allowed_origins = AllowedOrigins::all();
    let allowed_methods: AllowedMethods = ["GET", "POST"]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::build()
        .attach(cors)
        .mount("/", routes![hooks_verify, core])
        .launch()
        .await?;
    Ok(())
}
