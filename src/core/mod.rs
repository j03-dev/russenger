use std::str::FromStr;

use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::{catch, catchers, get, post, routes, State};
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};

use action::ACTION_REGISTRY;
use app_state::AppState;
use data::Data;
use incoming_data::InComingData;
use request::Req;
use response::Res as res;
use webhook_query::WebHookQuery;

use crate::payload::Payload;
use crate::query::Query;
use crate::text::TextModel;

pub mod action;
pub mod data;
pub mod request;
pub mod response;

mod app_state;
mod incoming_data;
mod webhook_query;

#[catch(404)]
fn page_not_found() -> &'static str {
    "Page not found: 404"
}

#[catch(500)]
fn server_panic() -> &'static str {
    "Server panic: 500"
}

#[get("/webhook")]
async fn webhook_verify(webhook_query: WebHookQuery) -> String {
    webhook_query.hub_challenge
}

async fn execute_payload(user: &str, uri: &str, query: &Query) {
    match Payload::from_uri_string(uri) {
        Ok(payload) => {
            if let Some(action) = ACTION_REGISTRY.lock().await.get(payload.get_path()) {
                let data = Data::from_string(payload.get_data_to_string());
                let request = Req::new(user, query.clone(), data);
                action.execute(res, request).await;
            }
        }
        Err(err) => {
            res.send(TextModel::new(user, &err)).await;
        }
    }
}

#[post("/webhook", format = "json", data = "<data>")]
async fn webhook_core(data: Json<InComingData>, app_state: &State<AppState>) -> &'static str {
    let query = &app_state.query;
    let user = data.get_sender();
    query.create(user).await;

    if let Some(message) = data.get_message() {
        let action_path = query.get_action(user).await.unwrap_or("Main".to_string());
        if let Some(quick_reply) = message.get_quick_reply() {
            let uri_payload = quick_reply.get_payload();
            execute_payload(user, uri_payload, query).await;
        } else if let Some(action) = ACTION_REGISTRY.lock().await.get(action_path.as_str()) {
            let request = Req::new(user, query.clone(), Data::new(message.get_text(), None));
            action.execute(res, request).await;
        }
    } else if let Some(postback) = data.get_postback() {
        let uri = postback.get_payload();
        execute_payload(user, uri, query).await;
    }

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

    rocket::build()
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
    println!("{}", migration_result);
}
