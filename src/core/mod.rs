use std::str::FromStr;

use rocket::serde::json::Json;
use rocket::{catch, catchers, get, post, routes, State};
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};

use action::ACTION_REGISTRY;
use request::Req;
use response::Res;

use crate::core::app_state::AppState;
use crate::core::data::Data;
use crate::core::deserializers::MessageDeserializer;
use crate::core::facebook_request::FacebookRequest;
use crate::query::Query;
use crate::response_models::payload::Payload;
use crate::response_models::text::TextModel;

pub mod action;
pub mod data;
pub mod request;
pub mod response;

mod app_state;
mod deserializers;
mod facebook_request;

#[catch(404)]
fn page_not_found() -> &'static str {
    "Page not found: 404"
}

#[catch(500)]
fn server_panic() -> &'static str {
    "Server panic: 500"
}

#[get("/webhook")]
async fn webhook_verify(request: FacebookRequest) -> String {
    request.0
}

async fn execute_payload(user: &str, data: &str, query: &Query) {
    match Payload::from_uri_string(data) {
        Ok(payload) => {
            if let Some(action_fn) = ACTION_REGISTRY
                .lock()
                .await
                .get(payload.get_action().as_str())
            {
                let data = payload.get_data_to_string();
                action_fn
                    .execute(Res, Req::new(user, query, Data::from(data)))
                    .await;
            }
        }
        Err(err) => {
            Res.send(TextModel::new(user, &err)).await.unwrap();
        }
    }
}

#[post("/webhook", format = "json", data = "<data>")]
async fn webhook_core(data: Json<MessageDeserializer>, state: &State<AppState>) -> &'static str {
    let user = data.get_sender();
    let query = &state.query;
    if let Some(message) = data.get_message() {
        query.create(user).await;
        let action = query.get_action(user).await.expect("failed to get action");
        if let Some(quick_reply) = message.get_quick_reply() {
            let uri_payload = quick_reply.get_payload();
            execute_payload(user, uri_payload, query).await;
        } else if action.ne("lock") {
            if let Some(action_fn) = ACTION_REGISTRY.lock().await.get(action.as_str()) {
                query.set_action(user, "lock").await;
                let data = message.get_text();
                action_fn
                    .execute(Res, Req::new(user, query, Data::new(data, None)))
                    .await;
            }
        } else {
            query.reset_action(user).await;
        }
    } else if let Some(postback) = data.get_postback() {
        let uri_payload = postback.get_payload();
        execute_payload(user, uri_payload, query).await;
    }
    "Ok"
}

pub async fn run_server() {
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
    .expect("Failed create cors: Some thing wrong on cors");

    rocket::build()
        .attach(cors)
        .manage(AppState::init().await)
        .mount("/", routes![webhook_verify, webhook_core])
        .register("/", catchers![page_not_found, server_panic])
        .launch()
        .await
        .expect("Failed run rocker server");
}

pub async fn migrate() {
    let query = Query::new().await;
    println!("Connexion Success");
    if query.migrate().await {
        println!("Migrate Success");
    } else {
        println!("Migrate Failed");
    }
}
