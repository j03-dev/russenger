use rocket::serde::json::{json, Value};

use super::payload::Payload;

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
#[derive(Clone, Debug)]
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
