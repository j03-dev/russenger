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
//! ## Macro
//!
//! - `action`: This proc macro is used to define an action.
//!
//! ## Deprecated
//!
//! - `create_action`: This macro is deprecated. Please use the `#[action]` proc macro instead. It was used to create a new action that sends a greeting message when the user input is "Hello".
//!
//! ## Examples
//!
//! Creating a new action that sends a greeting message when the user input is "Hello":
//!
//! ```rust
//! use russenger::create_action;
//! use russenger::prelude::*;
//!
//! create_action!(Main, |res: Res, req: Req| async move {
//!     let message: String = req.data.get_value();
//!     if message == "Hello" {
//!         res.send(TextModel::new(&req.user, "Hello, welcome to our bot!")).await;
//!     }
//! });
//! ```
//!
//! Creating an action using the new `#[action]` proc macro:
//!
//! ```rust
//! use russenger::prelude::*;
//!
//! #[action]
//! async fn Main(res: Res, req: Req) {
//!     let message: String = req.data.get_value();
//!     if message == "Hello" {
//!         res.send(TextModel::new(&req.user, "Hello, welcome to our bot!")).await;
//!     }
//! }
//! ```
pub use actix_web::main;
pub use async_trait::async_trait;
pub mod cli;
pub mod core;
pub mod prelude;
pub mod query;
pub mod response_models;

pub use cli::launch;
pub use core::action::{Action, ACTION_REGISTRY};
pub use dotenv::dotenv;
pub use russenger_macro::action;

/// # Deprecated
///
/// This macro is deprecated. Please use the `#[action]` proc macro instead.
///
/// ```rust
/// use russenger::prelude::*;
///
/// #[action]
/// async fn Main(res: Res, req: Req) {
///     let message: String = req.data.get_value();
///     if message == "Hello" {
///         res.send(TextModel::new(&req.user, "Hello, welcome to our bot!")).await;
///     }
/// }
/// ```
/// # Examples
///
/// Creating a new action that sends a greeting message when the user input is "Hello":
///
/// ```rust
/// use russenger::create_action;
/// use russenger::prelude::*;
///
/// create_action!(Main, |res: Res, req: Req| async move {
///     let message: String = req.data.get_value();
///     if message == "Hello" {
///         res.send(TextModel::new(&req.user, "Hello, welcome to our bot!")).await;
///     }
/// });
/// ```
#[macro_export]
#[deprecated(
    since = "0.1.5",
    note = "Please use the `#[action]` proc macro instead."
)]
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

/// The `russenger_app!` macro is used to create the main application.
///
/// It registers all the provided actions in the `ACTION_REGISTRY` and then starts the command handler.
///
/// # Syntax
///
/// ```rust
/// use russenger::prelude::*;
///
/// #[action]
/// async fn Action1(res: Res, req: Req) {
/// }
///
/// #[action]
/// async fn Action2(res: Res, req: Req) {
/// }
///
/// #[action]
/// async fn ActionN(res: Res, req: Req) {
/// }
///
/// russenger_app!(Action1, Action2, ActionN);
/// ```
///
/// * `action1, action2, ..., actionN`: The actions to be registered. These should be instances of structs that implement the `Action` trait.
///
/// # Examples
///
/// Creating the main application with the `Main` and `Greet` actions:
///
/// ```rust
/// use russenger::prelude::*;
///
/// #[action]
/// async fn Main(res: Res, req: Req) {
///     res.send(TextModel::new(&req.user, "welcome to our bot!")).await;
///     res.send(TextModel::new(&req.user, "What is your name: ")).await;
///     req.query.set_action(&req.user, Greet).await;
/// }
///
/// #[action]
/// async fn Greet(res: Res, req: Req) {
///     let name: String = req.data.get_value();
///     res.send(TextModel::new(&req.user, &format!("Hello : {name}"))).await;
/// }
///
/// russenger_app!(Main, Greet);
/// ```
#[macro_export]
macro_rules! russenger_app {
    ($($action:expr),* $(,)?) => {
        use russenger::{launch, Action, ACTION_REGISTRY};

        #[russenger::main]
        async fn main() {
            $(ACTION_REGISTRY.lock().await.insert($action.path(), Box::new($action));)*
            launch().await;
        }
    };
}
