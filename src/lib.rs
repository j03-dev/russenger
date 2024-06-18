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
//! - `russenger_app`: This macro is used to create the main application.
//!
//! ## Deprecated
//!
//! - `create_action`: This macro is deprecated. Please use the `#[action]` proc macro instead. It was used to create a new action that sends a greeting message when the user input is "Hello".
//!
//! ## Examples
//!
//! Creating a new action that sends a greeting message when the user input is "Hello": using the new `#[action]` proc macro:
//!
//! ```rust
//! use russenger::models::RussengerUser;
//! use russenger::prelude::*;
//!
//! #[action]
//! async fn Main(res: Res, req: Req) {
//!     let message: String = req.data.get_value();
//!     if message == "Hello" {
//!         res.send(TextModel::new(&req.user, "Hello, welcome to our bot!")).await;
//!     }
//! }
//!
//! #[russenger::main]
//! async fn main() {
//!     let conn = Database::new().await.conn;
//!     migrate!([RussengerUser], &conn);
//!     russenger::actions![Main];
//!     russenger::launch().await;
//! }
//! ```
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
pub use russenger_macro::action;


#[macro_export]
macro_rules! actions {
    [$($action:expr),* $(,)?] => {
        use russenger::{Action, ACTION_REGISTRY};
        $(ACTION_REGISTRY.lock().await.insert($action.path(), Box::new($action));)*
    };
}