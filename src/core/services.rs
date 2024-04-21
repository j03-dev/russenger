use std::str::FromStr;

use rocket::serde::json::Json;
use rocket::{catch, get, post, State};

use super::action::{ACTION_LOCK, ACTION_REGISTRY};
use super::app_state::AppState;
use super::data::Data;
use super::incoming_data::InComingData;
use super::request::Req;
use super::request_handler::{WebQuery, WebRequest};
use super::response::Res as res;

use crate::query::Query;
use crate::response_models::payload::Payload;

#[catch(404)]
pub fn page_not_found() -> &'static str {
    "Page not found: 404"
}

#[catch(500)]
pub fn server_panic() -> &'static str {
    "Server panic: 500"
}

#[get("/webhook")]
pub fn webhook_verify(webhook_query: WebQuery<'_>) -> &str {
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
                    action.execute(res, req).await;
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
pub async fn webhook_core(
    data: Json<InComingData>,
    app_state: &State<AppState>,
    request: WebRequest,
) -> &'static str {
    let query = app_state.query.clone();
    let user = data.get_sender();
    let host = request.host;
    query.create(user).await;
    if ACTION_LOCK.lock(user).await {
        if let Some(message) = data.get_message() {
            if let Some(quick_reply) = message.get_quick_reply() {
                let payload = quick_reply.get_payload();
                run(Executable::Payload(user, payload, &host, query)).await;
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
