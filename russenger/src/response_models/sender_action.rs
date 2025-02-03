//! This module provides `Actions` enum and `SenderActionModel` struct that represent sender actions in a Messenger conversation.
//!
//! ## Actions Enum
//!
//! The `Actions` enum is used to specify the type of sender action to send to the recipient. Sender actions include `MarkSeen`, `TypingOn`, and `TypingOff`. These actions allow you to control the status of the conversation, such as marking a message as seen or showing a typing indicator.
//!
//! ### Variants
//!
//! * `MarkSeen` - Marks the last message as seen. This will show a "seen" receipt to the user.
//! * `TypingOn` - Turns the typing indicators on. This will show a "typing..." indicator to the user.
//! * `TypingOff` - Turns the typing indicators off. This will hide the "typing..." indicator from the user.
//!
//! ## SenderActionModel Struct
//!
//! The `SenderActionModel` struct is used to send sender actions to the recipient. Sender actions include `mark_seen`, `typing_on`, and `typing_off`. These actions allow you to control the status of the conversation, such as marking a message as seen or showing a typing indicator.
//!
//! ### Fields
//!
//! * `messaging_type: String` - The type of messaging. For sender actions, this is always "RESPONSE".
//! * `recipient: Recipient` - The recipient of the sender action.
//! * `sender_action: String` - The sender action to send.
//!
//! ## Examples
//!
//! Using `Actions` and `SenderActionModel`:
//!
//! ```rust
//! use russenger::response_models::sender_action::{Actions, SenderActionModel};
//!
//! let action = SenderActionModel::new("sender_id", Actions::MarkSeen);
//! let action = SenderActionModel::new("sender_id", Actions::TypingOn);
//! let action = SenderActionModel::new("sender_id", Actions::TypingOff);
//! ```
//!
//! Use case:
//!
//! ```rust
//! use russenger::models::RussengerUser;
//! use russenger::prelude::*;
//!
//! async fn index(res: Res, req: Req) -> Result<()> {
//!     res.send(SenderActionModel::new(&req.user, MarkSeen)).await?;
//!     res.send(TextModel::new(&req.user, "Hello, world!")).await?;
//!
//!     Ok(())
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let conn = Database::new().await?.conn;
//!     migrate!([RussengerUser], &conn);
//!     App::init().await?
//!         .attach(router![("/", index)])
//!         .launch().await?;
//!     Ok(())
//! }
//! ```
//!
//!
//! ## Reference
//!
//! [Facebook Messenger Platform - Sender Actions](https://developers.facebook.com/docs/messenger-platform/send-messages/sender-actions)
use super::{recipient::Recipient, ResponseModel};
use serde::Serialize;

/// `Actions` is an enum used to specify the type of sender action to send to the recipient.
///
/// Sender actions include `MarkSeen`, `TypingOn`, and `TypingOff`.
///
/// These actions allow you to control the status of the conversation, such as marking a message as seen or showing a typing indicator.
///
/// # Variants
///
/// * `MarkSeen` - Marks the last message as seen. This will show a "seen" receipt to the user.
/// * `TypingOn` - Turns the typing indicators on. This will show a "typing..." indicator to the user.
/// * `TypingOff` - Turns the typing indicators off. This will hide the "typing..." indicator from the user.
///
/// # Examples
///
/// Using `Actions::MarkSeen` to mark a message as seen:
///
/// ```rust
/// use russenger::response_models::sender_action::{Actions, SenderActionModel};
/// let action = SenderActionModel::new("sender_id", Actions::MarkSeen);
/// ```
///
/// Using `Actions::TypingOn` to turn the typing indicators on:
///
/// ```rust
/// use russenger::response_models::sender_action::{Actions, SenderActionModel};
/// let action = SenderActionModel::new("sender_id", Actions::TypingOn);
/// ```
///
/// Using `Actions::TypingOff` to turn the typing indicators off:
///
/// ```rust
/// use russenger::response_models::sender_action::{Actions, SenderActionModel};
/// let action = SenderActionModel::new("sender_id", Actions::TypingOff);
/// ```
pub enum Actions {
    MarkSeen,
    TypingOn,
    TypingOff,
}

/// `SenderActionModel` is used to send sender actions to the recipient.
/// Sender actions include `mark_seen`, `typing_on`, and `typing_off`.
///
/// These actions allow you to control the status of the conversation, such as marking a message as seen or showing a typing indicator.
///
/// The `SenderActionModel` struct contains the following fields:
/// - `messaging_type`: A string that specifies the type of messaging. For sender actions, this is always "RESPONSE".
/// - `recipient`: A `Recipient` struct that specifies the recipient of the sender action.
/// - `sender_action`: A string that specifies the sender action to send.
///
/// # Examples
///
/// ``` rust
/// use russenger::prelude::*;
///
/// async fn index(res: Res, req: Req) -> Result<()> {
///     let action = SenderActionModel::new(&req.user, MarkSeen);
///     res.send(action).await?;
///
///     Ok(())
/// }
/// ```
///
/// Sending a `mark_seen` action:
///
/// ```rust
/// use russenger::response_models::sender_action::{Actions, SenderActionModel};
/// let action = SenderActionModel::new("sender_id", Actions::MarkSeen);
/// ```
///
/// Sending a `typing_on` action:
///
/// ```rust
/// use russenger::response_models::sender_action::{Actions, SenderActionModel};
/// let action = SenderActionModel::new("sender_id", Actions::TypingOn);
/// ```
///
/// Sending a `typing_off` action:
///
/// ```rust
/// use russenger::response_models::sender_action::{Actions, SenderActionModel};
/// let action = SenderActionModel::new("sender_id", Actions::TypingOff);
/// ```
///
/// [Facebook Documentation](https://developers.facebook.com/docs/messenger-platform/send-messages/sender-actions)
#[derive(Debug, Clone, Serialize)]
pub struct SenderActionModel<'a> {
    messaging_type: &'a str,
    recipient: Recipient<'a>,
    sender_action: &'a str,
}

impl<'a> SenderActionModel<'a> {
    /// Creates a new `SenderActionModel`.
    ///
    /// The `SenderActionModel` is used to send sender actions to the recipient. These actions include marking a message as seen, turning typing indicators on, and turning typing indicators off.
    ///
    /// # Arguments
    ///
    /// * `sender` - A string slice that holds the ID of the sender. This is the unique identifier for the user or page that will send the action.
    ///
    /// * `action` - An `Actions` enum that specifies the sender action to send. This can be one of the following:
    ///     * `Actions::MarkSeen` - Marks the last message as seen. This will show a "seen" receipt to the user.
    ///     * `Actions::TypingOn` - Turns the typing indicators on. This will show a "typing..." indicator to the user.
    ///     * `Actions::TypingOff` - Turns the typing indicators off. This will hide the "typing..." indicator from the user.
    ///
    /// # Returns
    ///
    /// This method returns a `SenderActionModel` instance with the `messaging_type` field set to "RESPONSE", the `recipient` field set to the provided sender ID, and the `sender_action` field set to the provided action.
    ///
    /// # Example
    ///
    /// ```rust
    /// use russenger::response_models::sender_action::{Actions, SenderActionModel};
    /// let action = SenderActionModel::new("sender_id", Actions::MarkSeen);
    /// ```
    pub fn new(sender: &'a str, action: Actions) -> Self {
        let sender_action = match action {
            Actions::MarkSeen => "mark_seen",
            Actions::TypingOn => "typing_on",
            Actions::TypingOff => "typing_off",
        };
        Self {
            messaging_type: "RESPONSE",
            recipient: Recipient { id: sender },
            sender_action,
        }
    }
}

impl ResponseModel for SenderActionModel<'_> {
    const END_POINT: &'static str = "messages";
}
