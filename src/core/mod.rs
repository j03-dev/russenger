use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use std::env;

use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::{catch, catchers, get, post, routes, State};
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};

use action::{ACTION_LOCK, ACTION_REGISTRY};
use app_state::AppState;
use data::Data;
use incoming_data::InComingData;
use request::Req;
use response::Res as res;

use crate::payload::Payload;
use crate::query::Query;
use crate::text::TextModel;

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

#[post("/webhook")]
async fn webhook_core(
    data: web::Json<InComingData>,
    app_state: web::Data<AppState>,
) -> &'static str {
    let query = &app_state.query;
    let user = data.get_sender();
    query.create(user).await;

    if ACTION_LOCK.lock(user).await {
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
    println!("{}", migration_result);
}
