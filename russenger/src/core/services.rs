//! The `services` contains the actix-web handlers for the webhook endpoints.
//!
//! # Functions
//!
//! * `webhook_verify`: This function verifies the webhook.
//! * `webhook_core`: This function handles the webhook core.
//!
//! Endpoints:
//!
//! * `GET /webhook`: This endpoint verifies the webhook.
//! * `POST /webhook`: This endpoint handles the webhook core.
use std::{str::FromStr, sync::Arc};

use crate::error::Result;
use actix_web::{dev, get, post, web, HttpResponse};
use tokio::sync::Mutex;

use super::{
    action::Router, app::App, incoming_data::InComingData, request::Req, request_handler::WebQuery,
    response::Res,
};

use crate::{
    query::Query,
    response_models::{data::Data, payload::Payload},
};

#[get("/webhook")]
pub async fn webhook_verify(web_query: web::Query<WebQuery>) -> HttpResponse {
    web_query.get_hub_challenge()
}

enum Message<'a> {
    Payload(&'a str, &'a str, &'a str, Query),
    TextMessage(&'a str, &'a str, &'a str, Query),
}

async fn handle(message: Message<'_>, router: Arc<Mutex<Router>>) -> Result<()> {
    match message {
        Message::Payload(user, payload, host, query) => {
            let payload = Payload::from_str(payload).unwrap_or_default();
            {
                let data = payload.get_data();
                let res = Res::new(user, query.clone());
                let req = Req::new(user, query, data, host);
                let path = payload.get_path();

                match router.lock().await.get(&path) {
                    Some(action) => action(res, req).await?,
                    None => {
                        eprintln!("Error 404: Action {:?} not found", path);
                    }
                }
            }
        }
        Message::TextMessage(user, text_message, host, query) => {
            let path = query.get_path(user).await.unwrap_or("/".to_string());
            let res = Res::new(user, query.clone());
            let req = Req::new(user, query, Data::new(text_message), host);
            match router.lock().await.get(&path) {
                Some(action) => action(res, req).await?,
                None => {
                    eprintln!("Error 404: Action {:?} not found", path);
                }
            }
        }
    }

    Ok(())
}

#[post("/webhook")]
pub async fn webhook_core(
    data: web::Json<InComingData>,
    app_state: web::Data<App>,
    conn: dev::ConnectionInfo,
) -> HttpResponse {
    let query = app_state.query.clone();
    let user = data.get_sender();
    let host = conn.host();
    query.create(user).await;

    if app_state.action_lock.lock(user).await {
        if let Some(message) = data.get_message() {
            if let Some(quick_reply) = message.get_quick_reply() {
                let payload = quick_reply.get_payload();
                if let Err(e) = handle(
                    Message::Payload(user, payload, host, query),
                    app_state.router.clone(),
                )
                .await
                {
                    eprintln!("Error handling payload: {:?}", e);
                }
            } else {
                let text = message.get_text();
                if let Err(e) = handle(
                    Message::TextMessage(user, &text, host, query),
                    app_state.router.clone(),
                )
                .await
                {
                    eprintln!("Error handling text message: {:?}", e);
                }
            }
        } else if let Some(postback) = data.get_postback() {
            let payload = postback.get_payload();
            if let Err(e) = handle(
                Message::Payload(user, payload, host, query),
                app_state.router.clone(),
            )
            .await
            {
                eprintln!("Error handling postback: {:?}", e);
            }
        }
    }
    app_state.action_lock.unlock(user).await;

    HttpResponse::Ok().finish()
}
