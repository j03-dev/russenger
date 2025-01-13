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

use crate::error::Context;
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

async fn handle(
    message: Message<'_>,
    router: &Arc<Mutex<Router>>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    match message {
        Message::Payload(user, payload, host, query) => {
            let payload = Payload::from_str(payload).unwrap_or_default();
            let data = payload.get_data();
            let res = Res::new(user, query.clone());
            let req = Req::new(user, query, data, host);
            let path = payload.get_path();

            let router = router.lock().await;
            let action = router
                .get(&path)
                .context("Action not found in the router")
                .map_err(|err| format!("Error finding action: {err}, path: {path}"))?;
            action(res, req).await?
        }
        Message::TextMessage(user, text_message, host, query) => {
            let path = query.get_path(user).await.unwrap_or("/".to_string());
            let res = Res::new(user, query.clone());
            let req = Req::new(user, query, Data::new(text_message), host);
            let router = router.lock().await;
            let action = router
                .get(&path)
                .context("Action not found in the router")
                .map_err(|err| format!("Error finding action: {err}, path: {path}"))?;
            action(res, req).await?
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
                let quick_reply_payload = quick_reply.get_payload();
                let payload = Message::Payload(user, quick_reply_payload, host, query);
                let result = handle(payload, &app_state.router).await;
                result.unwrap_or_else(|err| {
                    eprintln!("Error handling quick reply payload: {:?}", err)
                })
            } else {
                let text = message.get_text();
                let text_message = Message::TextMessage(user, &text, host, query);
                let result = handle(text_message, &app_state.router).await;
                result.unwrap_or_else(|err| eprintln!("Error handling text message: {:?}", err))
            }
        } else if let Some(postback) = data.get_postback() {
            let postback_playload = postback.get_payload();
            let payload = Message::Payload(user, postback_playload, host, query);
            let result = handle(payload, &app_state.router).await;
            result.unwrap_or_else(|err| eprintln!("Error handling postback payload: {:?}", err))
        }
        app_state.action_lock.unlock(user).await;
    } else {
        return HttpResponse::TooManyRequests().finish();
    }

    HttpResponse::Ok().finish()
}
