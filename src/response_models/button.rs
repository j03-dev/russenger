use rocket::serde::{
    json::{json, Value},
    Serialize,
};

use super::{payload::Payload, recipient::Recipient, ResponseModel};

/// `Button` is an enum that represents different types of buttons that can be used in a Messenger conversation.
///
/// # Variants
///
/// * `AccountUnlink` - Represents an account unlink button.
/// * `AccountLink { url: &'b str }` - Represents an account link button. The `url` field is the URL to be opened when the button is clicked.
/// * `WebUrl { title: &'b str, url: &'b str }` - Represents a web URL button. The `title` field is the title of the button, and the `url` field is the URL to be opened when the button is clicked.
/// * `Postback { title: &'b str, payload: Payload }` - Represents a postback button. The `title` field is the title of the button, and the `payload` field is the payload to be sent back to the server when the button is clicked.
/// * `PhoneNumber { title: &'b str, payload: Payload }` - Represents a phone number button. The `title` field is the title of the button, and the `payload` field is the phone number to be dialed when the button is clicked.
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
///     url: "https://www.facebook.com/your_account_page",
/// };
///
/// // Creating a WebUrl button
/// let web_url_button = Button::WebUrl {
///     title: "Visit Website",
///     url: "https://example.com",
/// };
///
/// // Creating a Postback button
/// let postback_button = Button::Postback {
///     title: "Click me",
///     payload: Payload::new(HelloWorld, Some(Data::new("HelloWorld", None))),
/// };
///
/// // Creating a PhoneNumber button
/// let phone_number_button = Button::PhoneNumber {
///     title: "Call me",
///     payload: Payload::new(HelloWorld, Some(Data::new("<PhoneNumber>", None))),
/// };
///
/// use russenger::prelude::*;
///
/// create_action!(HelloWorld, |res: Res, req: Req| async move {
///     let payload: String = req.data.get_value();
///     res.send(TextModel::new(&req.user, &payload)).await;
/// });
/// ```
#[derive(Clone, Debug, Serialize)]
pub enum Button<'b> {
    AccountUnlink,
    AccountLink { url: &'b str },
    WebUrl { title: &'b str, url: &'b str },
    Postback { title: &'b str, payload: Payload },
    PhoneNumber { title: &'b str, payload: Payload },
}

impl<'b> Button<'b> {
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
    buttons: Vec<Button<'p>>,
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
/// create_action!(Main, |res: Res, req: Req| async move {
///     let buttons = vec![
///         Button::WebUrl {title: "Click Me", url: "https://link.test.com"},
///         // More Button ...
///     ];
///     res.send(ButtonModel::new(&req.user, "Option", buttons)).await;
/// });
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
    ///     Button::WebUrl {title: "Click Me", url: "https://link.test.com"},
    ///     // More Button ...
    /// ];
    /// ButtonModel::new("sender_id", "Option", buttons);
    /// ```
    pub fn new(sender: &'b str, text: &'b str, buttons: Vec<Button>) -> Self {
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
