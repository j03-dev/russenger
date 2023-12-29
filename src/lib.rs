pub mod command;
pub mod core;
pub mod query;
pub mod response_models;

pub use crate::core::action::Action;
pub use crate::core::data::Data;
pub use crate::core::request::Req;
pub use crate::core::response::Res;
pub use rocket::{async_trait, main};

#[macro_export]
macro_rules! create_action {
    ($name:ident, $handler:expr) => {
        pub struct $name;

        #[russenger::async_trait]
        impl russenger::Action for $name {
            async fn execute<'l>(&self, res: russenger::Res, req: russenger::Req<'l>) {
                ($handler)(res, req).await;
            }
        }
    };
}

#[macro_export]
macro_rules! russenger_app {
    ($($action:expr),* $(,)?) => {
        use russenger::command::execute_command;
        use russenger::core::action::ACTION_REGISTRY;

        #[russenger::main]
        async fn main() {
            $(ACTION_REGISTRY.lock().await.insert($action.path().as_str(), Box::new($action));)*
            execute_command().await;
        }
    };
}
