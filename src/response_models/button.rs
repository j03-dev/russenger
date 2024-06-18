//! This module contains the `Button` enum and the `ButtonModel` struct.
//!
//! # Examples
//!
//! Creating a new button:
//!
//! ```rust
//! use russenger::models::RussengerUser;
//! use russenger::prelude::*;
//!
//! #[action]
//! async fn Main(res: Res, req: Req) {
//!     let payload = Payload::new(HelloWorld, Some(Data::new("HelloWorld", None)));
//!     let buttons = vec![
//!         Button::Postback { title: "Click me".to_owned(), payload: payload },
//!         // Add more buttons here
//!     ];
//!     let button_model = ButtonModel::new("sender_id", "Hello, user1!", buttons);
//!     res.send(button_model).await;
//! }
//! #[action]
//! async fn HelloWorld(res: Res, req: Req) {
//!     let value: String = req.data.get_value();
//!     res.send(TextModel::new(&req.user, &value)).await;
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
use serde::Serialize;
use serde_json::json;
use serde_json::value::Value;

use super::{payload::Payload, recipient::Recipient, ResponseModel};

/// `Button` is an enum that represents different types of buttons that can be used in a Messenger conversation.
///
/// # Variants
///
/// * `AccountUnlink` - Represents an account unlink button.
/// * `AccountLink { url: Stirng }` - Represents an account link button. The `url` field is the URL to be opened when the button is clicked.
/// * `WebUrl { title: Stirng, url: Stirng }` - Represents a web URL button. The `title` field is the title of the button, and the `url` field is the URL to be opened when the button is clicked.
/// * `Postback { title: Stirng, payload: Payload }` - Represents a postback button. The `title` field is the title of the button, and the `payload` field is the payload to be sent back to the server when the button is clicked.
/// * `PhoneNumber { title: Stirng, payload: Payload }` - Represents a phone number button. The `title` field is the title of the button, and the `payload` field is the phone number to be dialed when the button is clicked.
///
/// # Examples
///
/// Creating a `Button` and converting it to a `Value`:
///
/// ```rust
/// use russenger::response_models::data::Data;
/// use russenger::response_models::payload::Payload;
/// use russenger::response_models::button::Button;
// Creating an AccountUnlink button
/// let account_unlink_button = Button::AccountUnlink;
///
/// // Creating an AccountLink button
/// let account_link_button = Button::AccountLink {
///     url: "https://www.facebook.com/your_account_page".to_owned(),
/// };
///
/// // Creating a WebUrl button
/// let web_url_button = Button::WebUrl {
///     title: "Visit Website".to_owned(),
///     url: "https://example.com".to_owned(),
/// };
///
/// // Creating a Postback button
/// let postback_button = Button::Postback {
///     title: "Click me".to_owned(),
///     payload: Payload::new(HelloWorld, Some(Data::new("HelloWorld", None))),
/// };
///
/// // Creating a PhoneNumber button
/// let phone_number_button = Button::PhoneNumber {
///     title: "Call me".to_owned(),
///     payload: Payload::new(HelloWorld, Some(Data::new("<PhoneNumber>", None))),
/// };
///
/// use russenger::prelude::*;
///
/// #[action]
/// async fn HelloWorld(res: Res, req: Req) {
///     let payload: String = req.data.get_value();
///     res.send(TextModel::new(&req.user, &payload)).await;
/// }
/// ```
#[derive(Clone, Debug, Serialize)]
pub enum Button {
    AccountUnlink,
    AccountLink { url: String },
    WebUrl { title: String, url: String },
    Postback { title: String, payload: Payload },
    PhoneNumber { title: String, payload: Payload },
}

impl Button {
    pub fn to_value(&self) -> Value {
        match self.clone() {
            Self::AccountLink { url } => {
                json!({"type": "account_link", "url": url})
            }
            Self::AccountUnlink => {
                json!({"type": "account_unlink"})
            }
            Self::Postback { title, payload } | Self::PhoneNumber { title, payload } => json!({
                "type": "postback",
                "title": title,
                "payload": payload.to_string()
            }),
            Self::WebUrl { title, url } => json!({
                "type": "web_url",
                "title": title,
                "url": url
            }),
        }
    }
}

#[derive(Serialize)]
struct ButtonPayload<'p> {
    template_type: &'p str,
    text: &'p str,
    buttons: Vec<Value>,
}

#[derive(Serialize)]
struct ButtonAttachement<'a> {
    #[serde(rename = "type")]
    r#type: &'a str,
    payload: ButtonPayload<'a>,
}
/// The `ButtonModel` struct represents a button template message.
///
/// The button template sends a text message with up to three buttons attached. This template gives the message recipient different options to choose from, such as predefined answers to questions or actions to take.
///
/// # Fields
///
/// * `recipient`: The recipient of the message. This is a `Recipient` struct that contains the Facebook user ID of the recipient.
/// * `message`: The message to be sent. This is a JSON value that contains the button template.
///
/// # Methods
///
/// * `new`: This method creates a new `ButtonModel`. It takes a sender ID, a text, and a vector of buttons as arguments.
///
/// # Examples
///
/// Creating a new `ButtonModel` and sending it using the `res.send()` method:
///
/// ```rust
/// use russenger::prelude::*;
///
/// #[action]
/// async fn Main(res: Res, req: Req) {
///     let buttons = vec![
///         Button::WebUrl {title: "Click Me".to_owned(), url: "https://link.test.com".to_owned()},
///         // More Button ...
///     ];
///     res.send(ButtonModel::new(&req.user, "Option", buttons)).await;
/// }
/// ```
///
/// # References
///
/// * [Facebook Button Template Documentation](https://developers.facebook.com/docs/messenger-platform/send-messages/template/button)
#[derive(Serialize)]
pub struct ButtonModel<'b> {
    recipient: Recipient<'b>,
    message: Value,
}

impl<'b> ButtonModel<'b> {
    /// Creates a new `ButtonModel`.
    ///
    /// # Arguments
    ///
    /// * `sender`: The Facebook user ID of the recipient.
    /// * `text`: The text to describe the functionality of the buttons.
    /// * `buttons`: The list of buttons to be sent.
    ///
    /// # Returns
    ///
    /// * `ButtonModel`: The created `ButtonModel`.
    /// # Example
    /// ```rust
    /// use russenger::prelude::*;
    ///
    /// let buttons = vec![
    ///     Button::WebUrl {title: "Click Me".to_owned(), url: "https://link.test.com".to_owned()},
    ///     // More Button ...
    /// ];
    /// ButtonModel::new("sender_id", "Option", buttons);
    /// ```
    pub fn new(sender: &'b str, text: &'b str, buttons: Vec<Button>) -> Self {
        let buttons = buttons.iter().map(|button| button.to_value()).collect();
        Self {
            recipient: Recipient { id: sender },
            message: json!({
                "attachment": ButtonAttachement {
                    r#type: "template",
                    payload: ButtonPayload {
                        template_type: "button",
                        text ,
                        buttons
                    }
                }
            }),
        }
    }
}

impl ResponseModel for ButtonModel<'_> {
    const END_POINT: &'static str = "messages";
}
