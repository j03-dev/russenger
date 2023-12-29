pub use rocket::{async_trait, main};

pub use crate::core::action::Action;
pub use crate::core::data::Data;
pub use crate::core::request::Req;
pub use crate::core::response::Res;

pub mod command;
pub mod core;
pub mod query;
pub mod response_models;

#[macro_export]
macro_rules! action {
    ($path_action:expr, $handler:expr) => {{
        struct Action;

        #[russenger::async_trait]
        impl russenger::Action for Action {
            async fn execute<'l>(&self, res: russenger::Res, req: russenger::Req<'l>) {
                ($handler)(res, req).await;
            }

            fn path(&self) -> String {
                $path_action.to_string()
            }
        }
    }};
}

#[macro_export]
macro_rules! russenger_app {
    ($($action:expr),* $(,)?) => {
        use russenger::command::execute_command;
        use russenger::core::action::ACTION_REGISTRY;

        #[russenger::main]
        async fn main() {
            $(ACTION_REGISTRY.lock().await.insert($action.path(), Box::new($action));)*
            execute_command().await;
        }
    };
}
