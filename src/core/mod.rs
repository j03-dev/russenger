pub mod app_state;

use crate::core::app_state::AppState;
use crate::hooks::messages::FacebookMessage;
use crate::hooks::MessengerWebhookRequest;
use crate::models::User;

use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::{catch, State};
use rocket::{get, post};
use std::collections::HashMap;
use std::sync::Arc;

#[rocket::async_trait]
pub trait Action: Send + Sync {
    async fn execute(&self, user_id: &str, message: &str, user_conn: &User);
}

type ActionRegistryType = Arc<Mutex<HashMap<&'static str, Box<dyn Action>>>>;

lazy_static::lazy_static! {
    pub static ref ACTION_REGISTRY: ActionRegistryType = Arc::new(Mutex::new(HashMap::new()));
}

#[macro_export]
macro_rules! register_action {
    ($path:expr, $action:expr) => {
        ACTION_REGISTRY
            .lock()
            .await
            .insert($path, Box::new($action));
    };
}

#[catch(404)]
pub fn general_not_found() -> &'static str {
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

#[post("/webhook", format = "json", data = "<facebook_message>")]
pub async fn webhook_core(
    facebook_message: Json<FacebookMessage>,
    state: &State<AppState>,
) -> &'static str {
    let message = &facebook_message.get_message();
    let user_id = &facebook_message.get_sender();

    let user_conn = &state.user_conn;
    user_conn.create(user_id).await;

    let action = user_conn
        .get_action(user_id)
        .await
        .expect("failed to get action");

    if action.ne("lock") {
        if let Some(action_fn) = ACTION_REGISTRY.lock().await.get(action.as_str()) {
            user_conn.set_action(user_id, "lock").await;
            action_fn.execute(user_id, message, user_conn).await;
        }
    } else {
        user_conn.reset_action(user_id).await;
    }

    "Ok"
}

#[macro_export]
macro_rules! potato_app {
    ($($path:expr => $action:expr),* $(,)?) => {{
        // Include necessary imports
        use potato::core::app_state::AppState;
        use potato::core::{
            general_not_found, server_panic, webhook_core, webhook_verify, ACTION_REGISTRY,
        };
        use potato::register_action;

        use std::str::FromStr;
        use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins};

        $(register_action!($path, $action);)*

        let allowed_origins = AllowedOrigins::all();
        let allowed_methods: AllowedMethods = ["Get", "Post"]
            .iter()
            .map(|s| FromStr::from_str(s).unwrap())
            .collect();

        let cors = rocket_cors::CorsOptions {
            allowed_origins,
            allowed_methods,
            allowed_headers: AllowedHeaders::all(),
            allow_credentials: true,
            ..Default::default()
        }
        .to_cors()?;

        rocket::build()
            .attach(cors)
            .manage(AppState::init().await)
            .mount("/", routes![webhook_verify, webhook_core])
            .register("/", catchers![general_not_found, server_panic])
            .launch()
            .await?;

        Ok(())
    }};
}
