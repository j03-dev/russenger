pub mod action;
pub mod app_state;

use crate::core::app_state::AppState;
use crate::hooks::messages::MsgFromFb;
use crate::hooks::MessengerWebhookRequest;
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
    let user_id = &data.get_sender();
    let user_conn = &state.user_conn;
    let action = user_conn
        .get_action(user_id)
        .await
        .expect("failed to get action");

    if action.ne("lock") {
        if let Some(action_fn) = ACTION_REGISTRY.lock().await.get(action.as_str()) {
            if let Some(message) = data.get_message() {
                user_conn.create(user_id).await;
                let text = &message.text.clone();

                user_conn.set_action(user_id, "lock").await;
                action_fn.execute(user_id, text, user_conn).await;
            } else if let Some(postback) = data.get_postback() {
                let payload = postback.payload.clone();

                action_fn.execute(user_id, &payload, user_conn).await;
            }
        }
    } else {
        user_conn.reset_action(user_id).await;
    }

    "Ok"
}
