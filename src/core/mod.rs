//! The `core` module contains the core functionality of the application.
//!
//! It includes several submodules that handle actions, application state, requests, responses, and services.
//!
//! # Submodules
//!
//! * `action`: This module contains the `Action` trait and the `ACTION_REGISTRY`.
//! * `app_state`: This module contains the `AppState` struct that represents the state of the application.
//! * `request`: This module contains the `Req` struct that represents a request from a user.
//! * `response`: This module contains the `Res` struct that represents a response that can be sent to a user.
//! * `services`: This module contains various services that the application can use.
//! * `incoming_data`: This module contains the functionality to handle incoming data.
//! * `request_handler`: This module contains the functionality to handle requests.
//!
//! # Examples
//!
//! Creating a new action:
//!
//! ```rust
//! use russenger::prelude::*;
//!
//! #[action]
//! async fn Greet(res: Res, req: Req) {
//!     res.send(TextModel::new(&req.user, "Hello, welcome to our bot!")).await;
//! }
//! ```

pub mod action;
pub mod app_state;
pub mod request;
pub mod request_handler;
pub mod response;
pub mod services;

mod incoming_data;
