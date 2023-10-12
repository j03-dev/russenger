pub mod core;
pub mod hooks;
pub mod models;
pub mod response_models;

#[macro_export]
macro_rules! register_action {
    ($path:expr, $action:expr) => {
        ACTION_REGISTRY
            .lock()
            .await
            .insert($path, Box::new($action));
    };
}

#[macro_export]
macro_rules! russenger_app {
    ($($path:expr => $action:expr),* $(,)?) => {{
        use std::str::FromStr;
        use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins};

        use russenger::core::action::ACTION_REGISTRY;
        use russenger::core::app_state::AppState;
        use russenger::core::{
            page_not_found,
            server_panic,
            webhook_core,
            webhook_verify
        };
        use russenger::register_action;

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
            .register("/", catchers![page_not_found, server_panic])
            .launch()
            .await?;

        Ok(())
    }};
}
