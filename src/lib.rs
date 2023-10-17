pub mod core;
pub mod hooks;
pub mod models;
pub mod response_models;
pub mod cli;

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
    ($($path:expr => $action:expr),* $(,)?) => {
        #[macro_use]
        extern crate rocket;

        use russenger::cli::command;
        use russenger::core::action::ACTION_REGISTRY;
        use russenger::register_action;

        #[rocket::main]
        async fn main(){
            $(register_action!($path, $action);)*
            command().await;
        }
    };
}
