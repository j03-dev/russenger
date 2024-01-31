pub use dotenv::dotenv;
pub use rocket::{async_trait, main};

pub use crate::command::execute_command;
pub use crate::core::action::{Action, ACTION_REGISTRY};
pub use crate::core::data::Data;
pub use crate::core::request::Req;
pub use crate::core::response::{
    Res, SendResult,
    SendResult::{Error, Okey},
};
pub use crate::response_models::generic;
pub use crate::response_models::media;
pub use crate::response_models::payload;
pub use crate::response_models::quick_replies;
pub use crate::response_models::text;
pub use crate::response_models::NextPrevNavigation;

pub mod command;
mod core;
mod query;
mod response_models;

// Define a macro to create a new action.
// This macro takes a name and a handler function, and defines a new struct with the given name that implements the Action trait.
#[macro_export]
macro_rules! create_action {
    ($name:ident, $handler:expr) => {
        // Define a new struct with the given name
        pub struct $name;

        // Implement the Action trait for the new struct
        #[russenger::async_trait]
        impl russenger::Action for $name {
            // The execute method calls the handler function
            async fn execute(&self, res: russenger::Res, req: russenger::Req) {
                ($handler)(res, req).await;
            }

            // The path method returns the name of the struct as a string
            fn path(&self) -> String {
                stringify!($name).to_string()
            }
        }
    };
}

// Define a macro to create a new Russenger application.
// This macro takes a list of actions, and defines a main function that registers the actions and executes the command.
#[macro_export]
macro_rules! russenger_app {
    ($($action:expr),* $(,)?) => {
        // Import necessary items from the russenger crate
        use russenger::{execute_command, Action, ACTION_REGISTRY};

        // Define the main function
        #[russenger::main]
        async fn main() {
            // Register each action in the ACTION_REGISTRY
            $(ACTION_REGISTRY.lock().await.insert($action.path(), Box::new($action));)*
            // Execute the command
            execute_command().await;
        }
    };
}
