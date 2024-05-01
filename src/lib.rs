pub mod cli;
pub mod core;
pub mod prelude;
mod query;
pub mod response_models;
#[cfg(test)]
mod test;

pub use cli::command_handler;
pub use core::action::{Action, ACTION_REGISTRY};
pub use dotenv::dotenv;
pub use rocket::{async_trait, main};

#[macro_export]
macro_rules! create_action {
    ($name:ident, $handler:expr) => {
        pub struct $name;

        #[russenger::async_trait]
        impl russenger::Action for $name {
            async fn execute(&self, res: russenger::prelude::Res, req: russenger::prelude::Req) {
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
        use russenger::{command_handler, Action, ACTION_REGISTRY};

        #[russenger::main]
        async fn main() {
            $(ACTION_REGISTRY.lock().await.insert($action.path(), Box::new($action));)*
            command_handler().await;
        }
    };
}
