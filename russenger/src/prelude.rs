//! This allows users to include everything they need with a single `use` statement.
//!
//! # Re-exports
//!
//! * `Req`: A struct that represents a request from a user.
//! * `Res`, `SendResult`: A struct and a type alias that represent a response that can be sent to a user.
//! * `Button`, `Data`, `GenericElement`, `GenericModel`, `GetStartedModel`, `MediaModel`, `Payload`, `PersistentMenuModel`, `QuickReply`, `QuickReplyModel`, `SenderActionModel`, `TextModel`, `ResponseModel`: Various response models that can be sent to a user.
//!
//! # Examples
//!
//! Using the `prelude` module to include everything needed for a basic application:
//!
//! ```rust
//! use russenger::prelude::*;
//!
//! async fn index (res: Res, req: Req) -> Result<()> {
//!     let message: String = req.data.get_value();
//!     if message == "Hi" {
//!         res.send(TextModel::new(&req.user, "Hello, welcome to our bot!")).await?;
//!     }
//!
//!     Ok(())
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     App::init().await?
//!         .attach(Router::new().add("/", index))
//!         .launch().await?;
//!     Ok(())
//! }
//! ```
pub use crate::core::{request::Req, response::Res, router::Router};
pub use crate::error::{self, Result};
pub use crate::response_models::{
    button::{Button, ButtonModel},
    data::Data,
    generic::{GenericElement, GenericModel},
    get_started::GetStartedButtonModel,
    media::MediaModel,
    next::NextModel,
    payload::Payload,
    persistent_menu::PersistentMenuModel,
    quick_replies::{QuickReply, QuickReplyModel},
    sender_action::{Actions::*, SenderActionModel},
    text::TextModel,
    ResponseModel,
};
pub use crate::router;
pub use crate::rusql_alchemy::{self, prelude::*};
pub use crate::App;
