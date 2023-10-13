pub mod action;
pub mod app_state;

use crate::core::app_state::AppState;
use crate::hooks::messages::MsgFromFb;
use crate::hooks::MessengerWebhookRequest;
use crate::response_models::payload::Payload;
use crate::response_models::text::TextModel;
use crate::response_models::SendResponse;
use action::ACTION_REGISTRY;

use rocket::serde::json::Json;
use rocket::{catch, get, post, State};

#[catch(404)]
pub fn page_not_found() -> &'static str {
    "Page not found: 404"
}

#[catch(500)]
pub fn server_panic() -> &'static str {
    "Server panic: 500"
}

#[get("/webhook")]
pub async fn webhook_verify(request: MessengerWebhookRequest) -> String {
    request.0
}

#[post("/webhook", format = "json", data = "<data>")]
pub async fn webhook_core(data: Json<MsgFromFb>, state: &State<AppState>) -> &'static str {
    let user_id = data.get_sender();
    let user_conn = &state.user_conn;

    if let Some(message) = data.get_message() {
        let action = user_conn
            .get_action(user_id)
            .await
            .expect("failed to get action");
        if action.ne("lock") {
            if let Some(action_fn) = ACTION_REGISTRY.lock().await.get(action.as_str()) {
                user_conn.set_action(user_id, "lock").await;
                action_fn
                    .execute(user_id, &message.get_text(), user_conn)
                    .await;
            }
        } else {
            user_conn.reset_action(user_id).await;
        }
    } else if let Some(postback) = data.get_postback() {
        let uri_payload = postback.get_payload();

        match Payload::from_uri_string(uri_payload) {
            Ok(payload) => {
                if let Some(action_fn) = ACTION_REGISTRY
                    .lock()
                    .await
                    .get(payload.get_action().as_str())
                {
                    action_fn
                        .execute(user_id, &payload.get_value(), user_conn)
                        .await;
                }
            }
            Err(err) => {
                TextModel::new(user_id, &err).send().await.unwrap();
            }
        }
    }
    "Ok"
}
