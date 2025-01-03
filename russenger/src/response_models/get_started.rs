//! This module provides a `GetStartedModel` struct that represents the GET STARTED button in a Messenger conversation.
//!
//! ## GetStartedModel Struct
//!
//! The `GetStartedModel` struct represents the GET STARTED button in a Messenger conversation. This button is shown when the user talks to the bot for the first time.
//!
//! ### Fields
//!
//! * `get_started: String` - The payload of the GET STARTED button.
//!
//! ### Methods
//!
//! * `new(payload: Payload) -> Self` - Creates a new `GetStartedModel` instance.
//!
//! ## Examples
//!
//! Creating a `GetStartedModel` and sending it:
//!
//! ```rust
//! use russenger::prelude::*;
//!
//! #[action]
//! async fn index(res: Res, req: Req) -> Result<()> {
//!     res.send(GetStartedButtonModel::new(Payload::default())).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Returns
//!
//! A POST request to the Facebook API to send a media file using the Facebook URL.
//!
//! ## Reference
//!
//! [Facebook Messenger Platform - Get Started Button](https://developers.facebook.com/docs/messenger-platform/reference/messenger-profile-api/get-started-button)
use serde::Serialize;

use super::{payload::Payload, ResponseModel};

/// `GetStartedModel` is a struct that represents the GET STARTED button in a Messenger conversation.
///
/// This button is shown when the user talks to the bot for the first time.
///
/// # Fields
///
/// * `get_started: String` - The payload of the GET STARTED button.
///
/// # Methods
///
/// * `new(payload: Payload) -> Self` - Creates a new `GetStartedModel` instance.
///
/// # Examples
///
/// Creating a `GetStartedModel` and sending it:
///
/// ```rust
/// use russenger::prelude::*;
///
/// #[action]
/// async fn index(res: Res, req: Req) -> Result<()>{
///     res.send(GetStartedButtonModel::new(Payload::default())).await?;
///
///     Ok(())
/// }
/// ```
///
/// # Returns
///
/// A POST request to the Facebook API to send a media file using the Facebook URL.
///
/// # Reference
///
/// [Facebook Messenger Platform - Get Started Button](https://developers.facebook.com/docs/messenger-platform/reference/messenger-profile-api/get-started-button)
#[derive(Debug, Clone, Serialize)]
pub struct GetStartedButtonModel {
    get_started: serde_json::Value,
}

impl GetStartedButtonModel {
    /// Creates a new `GetStartedModel` instance.
    ///
    /// # Parameters
    ///
    /// * `payload: Payload` - The payload of the GET STARTED button.
    ///
    /// # Returns
    ///
    /// A new `GetStartedModel` instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use russenger::response_models::get_started::GetStartedButtonModel;
    /// use russenger::response_models::payload::Payload;
    ///
    /// GetStartedButtonModel::new(Payload::default());
    /// ```
    pub fn new(payload: Payload) -> Self {
        Self {
            get_started: serde_json::json!({"payload": payload.to_string()}),
        }
    }
}

impl ResponseModel for GetStartedButtonModel {
    const END_POINT: &'static str = "messenger_profile";
}
