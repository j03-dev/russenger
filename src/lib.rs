//! # Russenger Library
//!
//! The Russenger library provides a set of modules and macros to help you build bots using the Russenger bot framework.
//!
//! ## Modules
//!
//! - `cli`: This module provides command-line interface utilities.
//! - `core`: This module contains the core functionalities of the Russenger bot framework.
//! - `prelude`: This module re-exports important traits and structs for convenience.
//! - `query`: This module provides utilities for handling queries.
//! - `response_models`: This module contains models for different types of responses.
//!
//! ## Macros
//!
//! - `action`: This proc macro is used to define an action.
//! - `russenger::actions!`: This macro is used to register the actions for the main application.
//!
//! ## New Features
//!
//! - **Custom Models**: Developers can now use their own models with the Russenger library. This is made possible by the integration with rusql_alchemy, an ORM for sqlx. This means that models are defined in Rust code, eliminating the need to write SQL queries.
//!
//! ## Examples
//!
//! Creating a new action that sends a greeting message when the user input is "Hello":
//!
//! ```rust
//! use russenger::models::RussengerUser;
//! use russenger::prelude::*;
//!
//! #[derive(FromRow, Clone, Model)]
//! pub struct Register {
//!     #[model(primary_key = true)]
//!     pub id: Serial,
//!     #[model(foreign_key = "RussengerUser.facebook_user_id", unique = true, null = false)]
//!     pub user_id: String,
//!     #[model(size = 30, unique = true, null = false)]
//!     pub username: String,
//! }
//!
//! #[action]
//! async fn Main(res: Res, req: Req) {
//!     // Send a greeting message to the user
//!     res.send(TextModel::new(&req.user, "Hello!")).await;
//!
//!     // Check if the user is registered
//!     if let Some(user_register) = Register::get(kwargs!(user_id = req.user), &req.query.conn).await {
//!         // If the user is registered, send a personalized greeting message
//!         res.send(TextModel::new(&req.user, &format!("Hello {}", user_register.username)))
//!             .await;
//!     } else {
//!         // If the user is not registered, ask for their name
//!         res.send(TextModel::new(&req.user, "What is your name: "))
//!             .await;
//!         // Set the next action to SignUp
//!         req.query.set_action(&req.user, SignUp).await;
//!         return;
//!     }
//!
//!     // If the user is registered, set the next action to GetUserInput
//!     req.query.set_action(&req.user, GetUserInput).await;
//! }
//!
//! #[action]
//! async fn SignUp(res: Res, req: Req) {
//!     // Get the username from the user input
//!     let username: String = req.data.get_value();
//!
//!     // Try to create a new Register record for the user
//!     let message = if Register::create(kwargs!(user_id = req.user, username = username), &req.query.conn).await {
//!         "Register success"
//!     } else {
//!         "Register failed"
//!     };
//!
//!     // Send a message to the user indicating whether the registration was successful
//!     res.send(TextModel::new(&req.user, message)).await;
//!
//!     // Go back to the Main action
//!     Main.execute(res, req).await;
//! }
//!
//! #[action]
//! async fn GetUserInput(res: Res, req: Req) {
//!     // Define a closure that creates a new Payload for a given value
//!     let payload = |value: &str| Payload::new(NextAction, Some(Data::new(value, None)));
//!
//!     // Create a QuickReplyModel with two options: "blue" and "red"
//!     let quick_replies: Vec<QuickReply> = vec![
//!         QuickReply::new("blue", "", payload("blue")),
//!         QuickReply::new("red", "", payload("red")),
//!     ];
//!     let quick_reply_model = QuickReplyModel::new(&req.user, "choose one color", quick_replies);
//!
//!     // Send the QuickReplyModel to the user
//!     res.send(quick_reply_model).await;
//! }
//!
//! #[action]
//! async fn NextAction(res: Res, req: Req) {
//!     // Get the color chosen by the user
//!     let color: String = req.data.get_value();
//!
//!     // Send a message to the user confirming their choice
//!     res.send(TextModel::new(&req.user, &color)).await;
//!
//!     // Go back to the Main action
//!     Main.execute(res, req).await;
//! }
//! ```
//!
//! Registering actions for the main application:
//!
//! ```rust
//! #[russenger::main]
//! async fn main() {
//!     // Connect to the database
//!     let conn = Database::new().await.conn;
//!
//!     // Migrate the database schema
//!     migrate!([RussengerUser, Register], &conn);
//!
//!     // Register the actions for the main application
//!     russenger::actions![Main, SignUp, GetUserInput, NextAction];
//!
//!     // Launch the main application
//!     russenger::launch().await;
//! }
//! ```
//!
//! These examples demonstrate how to define an action, use custom models, and register actions for the main application.
pub use actix_web::main;
pub use async_trait::async_trait;

pub mod cli;
pub mod core;
pub mod models;
pub mod prelude;
pub mod query;
pub mod response_models;

pub use cli::launch;
pub use core::action::{Action, ACTION_REGISTRY};
pub use dotenv::dotenv;
pub use rusql_alchemy;
pub use russenger_macro::action;

/// The `actions!` macro is used to register actions for the main application.
///
/// # Syntax
///
/// ```rust
/// russenger::actions![Action1, Action2, ...];
/// ```
///
/// # Arguments
///
/// * `Action1`, `Action2`, ...: The actions to register. These should be instances of a struct that implements the `Action` trait.
///
/// # Example
///
/// ```rust
/// use russenger::models::RussengerUser;
/// use russenger::prelude::*;
///
/// #[action]
/// async fn Main(res: Res, req: Req) {
///     // ...
/// }
///
/// #[action]
/// async fn GetUserInput(res: Res, req: Req) {
///     // ...
/// }
///
/// #[russenger::main]
/// async fn main() {
///     let conn = Database::new().await.conn;
///     migrate!([RussengerUser], &conn);
///     russenger::actions![Main, GetUserInput];
///     russenger::launch().await;
/// }
/// ```
///
/// In this example, the `Main` and `GetUserInput` actions are registered using the `actions!` macro.
#[macro_export]
macro_rules! actions {
    [$($action:expr),* $(,)?] => {
        use russenger::{Action, ACTION_REGISTRY};
        $(ACTION_REGISTRY.lock().await.insert($action.path(), Box::new($action));)*
    };
}
