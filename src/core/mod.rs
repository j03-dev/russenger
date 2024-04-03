pub mod action;
pub mod data;
pub mod request;
pub mod response;

mod app_state;
mod incoming_data;
mod request_handler;

use std::env::var;
use std::str::FromStr;

use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::{catch, catchers, get, post, routes, State};
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};

use action::{ACTION_LOCK, ACTION_REGISTRY};
use app_state::AppState;
use data::Data;
use incoming_data::InComingData;
use request::Req;
use request_handler::{WebQuery, WebRequest};
use response::Res as res;

use crate::payload::Payload;
use crate::query::Query;

#[catch(404)]
fn page_not_found() -> &'static str {
    "Page not found: 404"
}

#[catch(500)]
fn server_panic() -> &'static str {
    "Server panic: 500"
}

#[get("/webhook")]
fn webhook_verify(webhook_query: WebQuery<'_>) -> &str {
    webhook_query.hub_challenge
}

pub enum Executable<'a> {
    Payload(&'a str, &'a str, &'a str, Query),
    TextMessage(&'a str, &'a str, &'a str, Query),
}

async fn run(executable: Executable<'_>) {
    match executable {
        Executable::Payload(user, payload, host, query) => {
            let payload = Payload::from_str(payload).unwrap_or_default();
            {
                let data = payload.get_data();
                let req = Req::new(user, query, data, host);
                if let Some(action) = ACTION_REGISTRY.lock().await.get(&payload.get_path()) {
                    println!("try to run payload");
                    action.execute(res, req).await;
                    println!("finish to run payload");
                }
            }
        }
        Executable::TextMessage(user, text_message, host, query) => {
            let action_path = query.get_action(user).await.unwrap_or("Main".to_string());
            let req = Req::new(user, query, Data::new(text_message, None), host);
            if let Some(action) = ACTION_REGISTRY.lock().await.get(&action_path) {
                action.execute(res, req).await;
            }
        }
    }
}

#[post("/webhook", format = "json", data = "<data>")]
async fn webhook_core(
    data: Json<InComingData>,
    app_state: &State<AppState>,
    request: WebRequest,
) -> &'static str {
    println!("{data:#?}");
    let query = app_state.query.clone();
    let user = data.get_sender();
    let host = request.host;
    query.create(user).await;
    if ACTION_LOCK.lock(user).await {
        if let Some(message) = data.get_message() {
            if let Some(quick_reply) = message.get_quick_reply() {
                let paylaod = quick_reply.get_payload();
                run(Executable::Payload(user, paylaod, &host, query)).await;
            } else {
                let text = message.get_text();
                run(Executable::TextMessage(user, &text, &host, query)).await;
            }
        } else if let Some(postback) = data.get_postback() {
            let payload = postback.get_payload();
            run(Executable::Payload(user, payload, &host, query)).await;
        }
    }
    ACTION_LOCK.unlock(user).await;
    "Ok"
}

pub async fn run_server() {
    if !ACTION_REGISTRY.lock().await.contains_key("Main") {
        panic!("The ACTION_REGISTRY should contain an action with path 'Main' implementing the Action trait.");
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
    .expect("Failed to create CORS: Something went wrong with CORS");

    let port: i32 = var("PORT")
        .unwrap_or("2424".into())
        .parse()
        .expect("Should Containt number");
    let addr = var("HOST").unwrap_or("0.0.0.0".into());

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

pub async fn migrate() {
    let query = Query::new().await;
    println!("Connection successful!");
    let migration_result = match query.migrate().await {
        true => "Migration successful!",
        false => "Migration failed",
    };
    println!("{migration_result}");
}
