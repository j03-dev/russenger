use std::str::FromStr;

use rocket::{catch, catchers, get, post, routes, State};
use rocket::serde::json::Json;
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};

use action::ACTION_REGISTRY;

use crate::core::app_state::AppState;
use crate::core::callback::CallBackRequestVerification;
use crate::core::message_deserializer::MessageDeserializer;
use crate::query::Query;
use crate::response_models::payload::Payload;
use crate::response_models::SendResponse;
use crate::response_models::text::TextModel;

pub mod action;
mod app_state;
mod message_deserializer;
mod callback;

#[catch(404)]
fn page_not_found() -> &'static str {
    "Page not found: 404"
}

#[catch(500)]
fn server_panic() -> &'static str {
    "Server panic: 500"
}

#[get("/webhook")]
async fn webhook_verify(request: CallBackRequestVerification) -> String {
    request.0
}

async fn execute_payload(user_id: &str, uri_payload: &str, query: &Query) {
    match Payload::from_uri_string(uri_payload) {
        Ok(payload) => {
            if let Some(action_fn) = ACTION_REGISTRY
                .lock()
                .await
                .get(payload.get_action().as_str())
            {
                action_fn
                    .execute(user_id, &payload.get_value(), query)
                    .await;
            }
        }
        Err(err) => {
            TextModel::new(user_id, &err).send().await.unwrap();
        }
    }
}

#[post("/webhook", format = "json", data = "<data>")]
async fn webhook_core(data: Json<MessageDeserializer>, state: &State<AppState>) -> &'static str {
    let user_id = data.get_sender();
    let query = &state.query;
    let action = query
        .get_action(user_id)
        .await
        .expect("failed to get action");
    if let Some(message) = data.get_message() {
        if let Some(quick_reply) = message.get_quick_reply() {
            let uri_payload = quick_reply.get_payload();
            execute_payload(user_id, uri_payload, query).await;
        } else if action.ne("lock") {
            if let Some(action_fn) = ACTION_REGISTRY.lock().await.get(action.as_str()) {
                query.set_action(user_id, "lock").await;
                action_fn
                    .execute(user_id, &message.get_text(), query)
                    .await;
            }
        } else {
            query.reset_action(user_id).await;
        }
    } else if let Some(postback) = data.get_postback() {
        let uri_payload = postback.get_payload();
        execute_payload(user_id, uri_payload, query).await;
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
    let user_conn = Query::new().await;
    println!("Connexion Success");
    let status = user_conn.migrate().await;
    if status {
        println!("Migrate Success");
    } else {
        println!("Migrate Failed");
    }
}
