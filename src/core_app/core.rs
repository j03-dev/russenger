use crate::hooks::messages::FacebookMessage;
use rocket::serde::json::Json;
use crate::core_app::app_state::AppState;
use rocket::State;
use std::collections::HashMap;
use rocket::{get, post};

use crate::models::User;
use rocket::catcher::BoxFuture;
use crate::hooks::MessengerWebhookRequest;

pub type ActionFn = fn(&str, &str, &User) -> BoxFuture<'static, ()>;

// Create a lazy_static ActionRegistry to store actions and functions
lazy_static::lazy_static! {
    pub static ref ACTION_REGISTRY: HashMap<&'static str, ActionFn> = HashMap::new();
}

#[get("/webhook")]
pub async fn webhook_verify(request: MessengerWebhookRequest) -> String {
	request.0
}

#[post("/webhook", format="json", data="<facebook_message>")]
pub async fn webhook_core(facebook_message: Json<FacebookMessage>, state: &State<AppState>) -> &'static str {
	let message = &facebook_message.get_message();
	let user_id = &facebook_message.get_sender();

	let user_conn = &state.user_conn;
	user_conn.create(user_id).await;

	let action = user_conn.get_action(user_id).await.expect("failed to get action");

	if let Some(action_fn) = ACTION_REGISTRY.get(action.as_str()) {
        action_fn(user_id, message, user_conn).await;
    } else {
        user_conn.reset_action(user_id).await;
    }

	"Ok"
}
