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

/// Russenger
///
/// This is a Rust lib for building Messenger bots. It provides a set of core components for receiving and sending messages, handling actions, and managing payloads and responses.
///
/// ## Features
///
/// - **Action Handling**: Define actions with the `create_action!` macro and register them in the `ACTION_REGISTRY`. Actions are executed based on incoming requests.
/// - **Request and Response Handling**: `Req` and `Res` structs for handling incoming requests and outgoing responses.
/// - **Message Models**: Various response models for different types of messages (text, generic, media, quick replies, etc.).
///
/// ## Usage
///
/// Add the following to your `Cargo.toml`:
///
/// ```toml
/// [dependencies]
/// russenger = "0.2.0"
/// ```
///
/// Here's a simple example of a bot that sends a "Hello World" message:
///
/// ```rust
/// use russenger::{Req, Res, create_action, russenger_app};
/// use russenger::text::TextModel;
///
/// create_action!(Main, |res: Res, req: Req| async move {
///     res.send_text(TextModel::new(&req.user, "Hello World")).await;
///     Second.execute(res, req).await;
/// });
///
/// create_action!(Second, |res: Res, req: Req| async move {
/// 	todo!()
/// });
///
/// russenger_app!(Main, Second);
/// ```
///
/// Here's another example of a bot that sends a generic message:
///
/// ```rust
/// use russenger::{Req, Res, create_action, russenger_app, generic::GenericModel};
///
/// create_action!(Main, |res: Res, req: Req| async move {
/// 	todo!()
/// });
///
/// russenger_app!(Main);
/// ```

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
