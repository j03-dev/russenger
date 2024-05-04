use serde::Serialize;

use super::recipient::Recipient;
use super::ResponseModel;

#[derive(Serialize)]
struct Text<'t> {
    text: &'t str,
}

/// `TextModel` is used to send text messages to the recipient.
///
/// The `TextModel` struct contains the following fields:
/// - `recipient`: A `Recipient` struct that specifies the recipient of the text message.
/// - `messaging_type`: A string that specifies the type of messaging. For text messages, this is always "RESPONSE".
/// - `message`: A `Text` struct that contains the text of the message.
///
/// # Methods
///
/// * `new(sender: &'s str, text: &'s str) -> Self` - Creates a new `TextModel` instance.
///
/// # Examples
///
/// Sending a text message:
///
/// ```rust
/// use russenger::prelude::*;
///
/// create_action!(Main, |res: Res, req: Req| async move {
///     res.send(TextModel::new(&req.user, "Hello World!")).await
/// });
/// ```
///
/// ```rust
/// use russenger::response_models::text::TextModel;
/// let message = TextModel::new("sender_id", "Hello, world!");
/// ```
///
/// [Facebook Documentation](https://developers.facebook.com/docs/messenger-platform/send-messages/text)
#[derive(Serialize)]
pub struct TextModel<'s> {
    recipient: Recipient<'s>,
    messaging_type: &'s str,
    message: Text<'s>,
}

impl<'s> TextModel<'s> {
    /// Creates a new `TextModel`.
    ///
    /// This method allows you to send a text message to the given recipient. Note that the number of characters to send is limited to 2000 characters.
    ///
    /// # Arguments
    ///
    /// * `sender` - A string slice that holds the ID of the sender. This is the unique identifier for the user or page that will send the text message.
    /// * `text` - A string slice that holds the text of the message. This should be less than 2000 characters.
    ///
    /// # Returns
    ///
    /// This method returns a `TextModel` instance with the `recipient` field set to the provided sender ID, the `messaging_type` field set to "RESPONSE", and the `message` field set to the provided text.
    ///
    /// # Example
    ///
    /// ```rust
    /// use russenger::response_models::text::TextModel;
    /// let message = TextModel::new("sender_id", "Hello, world!");
    /// ```
    ///
    /// [Facebook Documentation](https://developers.facebook.com/docs/messenger-platform/send-messages#sending_text)
    pub fn new(sender: &'s str, text: &'s str) -> Self {
        Self {
            recipient: Recipient { id: sender },
            messaging_type: "RESPONSE",
            message: Text { text },
        }
    }
}

impl ResponseModel for TextModel<'_> {
    const END_POINT: &'static str = "messages";
}
