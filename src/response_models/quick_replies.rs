//! This module provides `QuickReply` and `QuickReplyModel` structs that represent a quick reply button and a message with quick reply buttons in a Messenger conversation.
//!
//! ## QuickReply Struct
//!
//! The `QuickReply` struct represents a quick reply button in a Messenger conversation. Quick replies provide a way to present a set of up to 13 buttons in-conversation that contain a title and optional image, and appear prominently above the composer.
//!
//! ### Fields
//!
//! * `content_type: String` - The type of content. In this case, it's always "text".
//! * `title: String` - The title of the quick reply button.
//! * `payload: String` - The payload of the quick reply button.
//! * `image_url: String` - The URL of the image to be displayed on the quick reply button.
//!
//! ### Methods
//!
//! * `new(title: &str, image_url: &str, payload: Payload) -> Self` - Creates a new `QuickReply` instance.
//!
//! ## QuickReplyModel Struct
//!
//! The `QuickReplyModel` struct represents a message with quick reply buttons in a Messenger conversation.
//!
//! ### Fields
//!
//! * `messaging_type: String` - The type of messaging. For quick reply messages, this is always "RESPONSE".
//! * `recipient: Recipient` - The recipient of the quick reply message.
//! * `message: QuickMessage` - The message that contains the text and the quick reply buttons.
//!
//! ### Methods
//!
//! * `new(sender: &'q str, text: &'q str, quick_replies: Vec<QuickReply>) -> Self` - Creates a new `QuickReplyModel` instance.
//!
//! ## Examples
//!
//! Creating a `QuickReply` and a `QuickReplyModel`:
//!
//! ```rust
//! use russenger::models::RussengerUser;
//! use russenger::prelude::*;
//! 
//! #[action]
//! async fn Main(res: Res, req: Req) {
//!     let data = Data::new("HelloWorld", None);
//!     let payload = Payload::new(HelloWorld, Some(data));
//!     let quick_replies = vec![QuickReply::new("Button Title", "https://example.com/image.png", payload)];
//!     let quick_reply = QuickReplyModel::new(&req.user, "Choose an option:", quick_replies);
//!     res.send(quick_reply).await;
//! }
//!
//!
//! #[action]
//! async fn HelloWorld(res: Res, req: Req) {
//!     let hello_world: String = req.data.get_value();
//!     res.send(TextModel::new(&req.user, &hello_world)).await;
//! }
//! 
//! #[russenger::main]
//! async fn main() {
//!     let conn = Database::new().await.conn;
//!     migrate!([RussengerUser], &conn);
//!     russenger::actions![Main, HelloWorld];
//!     russenger::launch().await;
//! }
//! ```
//!
//! ## Returns
//!
//! A POST request to the Facebook API to send a quick reply button or a message with quick reply buttons.
//!
//! ## Reference
//!
//! [Facebook Messenger Platform - Quick Replies](https://developers.facebook.com/docs/messenger-platform/send-messages/quick-replies)
use serde::Serialize;

use super::ResponseModel;
use super::{payload::Payload, recipient::Recipient};

/// `QuickReply` is a struct that represents a quick reply button in a Messenger conversation.
///
/// Quick replies provide a way to present a set of up to 13 buttons in-conversation that contain a title and optional image, and appear prominently above the composer.
///
/// # Fields
///
/// * `content_type: String` - The type of content. In this case, it's always "text".
/// * `title: String` - The title of the quick reply button.
/// * `payload: String` - The payload of the quick reply button.
/// * `image_url: String` - The URL of the image to be displayed on the quick reply button.
///
/// # Methods
///
/// * `new(title: &str, image_url: &str, payload: Payload) -> Self` - Creates a new `QuickReply` instance.
///
/// # Examples
///
/// Creating a `QuickReply`:
///
/// ```rust
/// use russenger::prelude::*;
///
/// let data = Data::new("HelloWorld", None);
/// let payload = Payload::new(HelloWorld, Some(data));
/// let quick_reply = QuickReply::new("Button Title", "https://example.com/image.png", payload);
///
/// #[action]
/// async fn HelloWorld(res: Res, req: Req) {
///     let hello_world: String = req.data.get_value();
///     res.send(TextModel::new(&req.user, &hello_world)).await;
/// }
/// ```
#[derive(Serialize, Debug)]
pub struct QuickReply {
    content_type: String,
    title: String,
    payload: String,
    image_url: String,
}

impl QuickReply {
    /// Creates a new `QuickReply` instance.
    ///
    /// # Parameters
    ///
    /// * `title: &str` - The title of the quick reply button.
    /// * `image_url: &str` - The URL of the image to be displayed on the quick reply button.
    /// * `payload: Payload` - The payload of the quick reply button.
    ///
    /// # Returns
    ///
    /// A new `QuickReply` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use russenger::prelude::*;
    /// let payload = Payload::new(SomeAction, None);
    /// let quick_reply = QuickReply::new("Button Title", "https://example.com/image.png", payload);
    ///
    /// #[action]
    /// async fn SomeAction(res: Res, req: Req) {
    ///     res.send(TextModel::new(&req.user, "SomeAction")).await;
    /// }
    /// ```
    pub fn new(title: &str, image_url: &str, payload: Payload) -> Self {
        Self {
            content_type: "text".to_owned(),
            title: title.to_owned(),
            payload: payload.to_string(),
            image_url: image_url.to_owned(),
        }
    }
}

#[derive(Serialize, Debug)]
struct QuickMessage {
    text: String,
    quick_replies: Vec<QuickReply>,
}

/// `QuickReplyModel` is a struct that represents a message with quick reply buttons in a Messenger conversation.
///
/// # Fields
///
/// * `recipient: Recipient<'q>` - The recipient of the message.
/// * `messaging_type: &'q str` - The type of messaging. In this case, it's always "RESPONSE".
/// * `message: QuickMessage` - The message with quick reply buttons.
///
/// # Methods
///
/// * `new(sender: &'q str, message: &str, quick_replies: Vec<QuickReply>) -> Self` - Creates a new `QuickReplyModel` instance.
///
/// # Examples
///
/// Creating a `QuickReplyModel`:
///
/// ```rust
/// use russenger::prelude::*;
///
/// #[action]
/// async fn SomeAction(res: Res, req: Req) {
///     let payload = Payload::new(SomeAction, None);
///     let quick_reply = QuickReply::new("Button Title", "https://example.com/image.png", payload);
///     let quick_reply_model = QuickReplyModel::new(&req.user, "Message Text", vec![quick_reply]);
///     res.send(quick_reply_model).await;
/// }
/// ```

#[derive(Debug, Serialize)]
pub struct QuickReplyModel<'q> {
    recipient: Recipient<'q>,
    messaging_type: &'q str,
    message: QuickMessage,
}

impl<'q> QuickReplyModel<'q> {
    /// Creates a new `QuickReplyModel` instance.
    ///
    /// # Parameters
    ///
    /// * `sender: &'q str` - The user ID of the recipient.
    /// * `message: &str` - The message text.
    /// * `quick_replies: Vec<QuickReply>` - The quick reply buttons.
    ///
    /// # Returns
    ///
    /// A new `QuickReplyModel` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use russenger::prelude::*;
    ///
    /// let payload = Payload::new(SomeAction, None);
    /// let quick_reply = QuickReply::new("Button Title", "https://example.com/image.png", payload);
    /// let quick_reply_model = QuickReplyModel::new("send_id", "Message Text", vec![quick_reply]);
    ///
    /// #[action]
    /// async fn SomeAction(res: Res, req: Req) {
    ///     res.send(TextModel::new(&req.user, "SomeAction")).await;
    /// }
    /// ```
    pub fn new(sender: &'q str, message: &str, quick_replies: Vec<QuickReply>) -> Self {
        Self {
            recipient: Recipient { id: sender },
            messaging_type: "RESPONSE",
            message: QuickMessage {
                text: message.into(),
                quick_replies,
            },
        }
    }
}

impl ResponseModel for QuickReplyModel<'_> {
    const END_POINT: &'static str = "messages";
}
