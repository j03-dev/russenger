use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use std::env;

use action::{ACTION_LOCK, ACTION_REGISTRY};
use app_state::AppState;
use data::Data;
use incoming_data::InComingData;
use request::Req;
use request_handler::{WebQuery, WebRequest};
use response::Res as res;

use crate::payload::Payload;
use crate::query::Query;

pub mod action;
pub mod data;
pub mod request;
pub mod response;

mod app_state;
mod incoming_data;

#[derive(Debug, Deserialize)]
struct WebhookQuery {
    #[serde(rename = "hub.mode")]
    hub_mode: Option<String>,
    #[serde(rename = "hub.challenge")]
    hub_challenge: Option<String>,
    #[serde(rename = "hub.verify_token")]
    hub_verify_token: Option<String>,
}

impl WebhookQuery {
    fn verify(&self) -> HttpResponse {
        match (
            self.hub_mode.clone(),
            self.hub_challenge.clone(),
            self.hub_verify_token.clone(),
        ) {
            (Some(hub_mode), Some(hub_challenge), Some(token)) => {
                if hub_mode.eq("subscribe") && env::var("VERIFY_TOKEN").unwrap().eq(&token) {
                    HttpResponse::Ok().body(hub_challenge)
                } else {
                    HttpResponse::Unauthorized().body("Token mismatch")
                }
            }
            _ => HttpResponse::Unauthorized().body("Arguments not enough"),
        }
    }
}

#[get("/webhook")]
async fn webhook_verify(query: web::Query<WebhookQuery>) -> HttpResponse {
    query.verify()
}

pub enum Executable<'a> {
    Payload(&'a str, &'a str, &'a str, Query),
    TextMessage(&'a str, &'a str, &'a str, Query),
}

async fn run(executable: Executable<'_>) {
    match executable {
        Executable::Payload(user, uri_payload, host, query) => {
            let payload = Payload::from_uri_string(uri_payload).unwrap_or_default();
            {
                let data = Data::from_string(payload.get_data_to_string());
                let req = Req::new(user, query, data, host);
                if let Some(action) = ACTION_REGISTRY.lock().await.get(payload.get_path()) {
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

#[post("/webhook")]
async fn webhook_core(
    data: web::Json<InComingData>,
    app_state: web::Data<AppState>,
) -> &'static str {
    let query = &app_state.query;
    let user = data.get_sender();
    let host = request.host;
    query.create(user).await;
    if ACTION_LOCK.lock(user).await {
        if let Some(message) = data.get_message() {
            if let Some(quick_reply) = message.get_quick_reply() {
                let paylaod = quick_reply.get_payload();
                run(Executable::Payload(user, &paylaod, &host, query)).await;
            } else {
                let text = message.get_text();
                run(Executable::TextMessage(user, &text, &host, query)).await;
            }
        } else if let Some(postback) = data.get_postback() {
            let payload = postback.get_payload();
            run(Executable::Payload(user, &payload, &host, query)).await;
        }
    }
    ACTION_LOCK.unlock(user).await;
    "Ok"
}

pub async fn run_server() {
    if !ACTION_REGISTRY.lock().await.contains_key("Main") {
        panic!("The ACTION_REGISTRY should contain an action with path 'Main' implementing the Action trait.");
    }
    let app_state = AppState::init().await;
    let host = env::var("HOST").unwrap_or("127.0.0.1".into());
    let port = env::var("PORT")
        .unwrap_or("8080".into())
        .parse()
        .unwrap_or(8080);
    println!("server start on {host}:{port}");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(webhook_verify)
            .service(webhook_core)
            .service(fs::Files::new("/static", ".").show_files_listing())
    })
    .bind((host.clone(), port))
    .expect("Failed to run this server: pls check the port if it's already used!")
    .run()
    .await
    .expect("sever is crashed");
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
