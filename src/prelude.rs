//! The `prelude` module re-exports the most commonly used types, traits, and functions.
//!
//! This allows users to include everything they need with a single `use` statement.
//!
//! # Re-exports
//!
//! * `Req`: A struct that represents a request from a user.
//! * `Res`, `SendResult`: A struct and a type alias that represent a response that can be sent to a user.
//! * `Button`, `Data`, `GenericElement`, `GenericModel`, `GetStartedModel`, `MediaModel`, `Payload`, `PersistentMenuModel`, `QuickReply`, `QuickReplyModel`, `SenderActionModel`, `TextModel`, `ResponseModel`: Various response models that can be sent to a user.
//! * `create_action`, `russenger_app`: Macros for creating actions and the main application.
//!
//! # Examples
//!
//! Using the `prelude` module to include everything needed for a basic application:
//!
//! ```rust
//! use russenger::prelude::*;
//!
//! create_action!(Main, |res: Res, req: Req| async move {
//!     let message: String = req.data.get_value();
//!     if message == "Hi" {
//!         res.send(TextModel::new(&req.user, "Hello, welcome to our bot!")).await;
//!     }
//! });
//!
//! russenger_app!(Main);
pub use crate::core::{
    request::Req,
    response::{Res, SendResult},
};
pub use crate::response_models::{
    button::Button,
    data::Data,
    generic::{GenericElement, GenericModel},
    get_started::GetStartedModel,
    media::MediaModel,
    payload::Payload,
    persistent_menu::PersistentMenuModel,
    quick_replies::{QuickReply, QuickReplyModel},
    sender_action::{Actions::*, SenderActionModel},
    text::TextModel,
    ResponseModel,
};
pub use crate::{create_action, russenger_app};
