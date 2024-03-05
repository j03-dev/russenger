pub use actix_web::main;
pub use async_trait::async_trait;
pub use dotenv::dotenv;

pub use command::execute_command;
pub use core::{
    action::{Action, ACTION_REGISTRY},
    data::Data,
    request::Req,
    response::{Res, SendResult},
};
pub use response_models::{generic, media, payload, quick_replies, text};

pub mod command;
mod core;
mod query;
mod response_models;

#[macro_export]
macro_rules! create_action {
    ($name:ident, $handler:expr) => {
        pub struct $name;

        #[russenger::async_trait]
        impl russenger::Action for $name {
            async fn execute(&self, res: russenger::Res, req: russenger::Req) {
                ($handler)(res, req).await;
            }

            fn path(&self) -> String {
                stringify!($name).to_string()
            }
        }
    };
}

#[macro_export]
macro_rules! russenger_app {
    ($($action:expr),* $(,)?) => {
        use russenger::{execute_command, Action, ACTION_REGISTRY};

        #[russenger::main]
        async fn main() {
            $(ACTION_REGISTRY.lock().await.insert($action.path(), Box::new($action));)*
            execute_command().await;
        }
    };
}
