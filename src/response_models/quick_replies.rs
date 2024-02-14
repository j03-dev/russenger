use rocket::serde::Serialize;

use super::{payload::Payload, recipient::Recipient};
use super::{GetSender, NextPrevNavigation};

#[derive(Serialize, Debug)]
pub struct QuickReply {
    content_type: String,
    title: String,
    payload: String,
    image_url: String,
}

impl QuickReply {
    pub fn new(title: &str, image_url: &str, payload: Payload) -> Self {
        Self {
            content_type: "text".into(),
            title: title.into(),
            payload: payload.to_uri_string(),
            image_url: image_url.into(),
        }
    }
}

#[derive(Serialize, Debug)]
struct QuickMessage {
    text: String,
    quick_replies: Vec<QuickReply>,
}

/// `QuickReplyModel` represents a quick reply message model.
///
/// It has a recipient, a messaging type, and a message. The message is a `QuickMessage` which contains a text and a vector of `QuickReply`.
///
/// # Example
///
/// ```rust
/// use russenger::payload::Payload;
/// use russenger::quick_replies::{QuickReply, QuickReplyModel};
/// use russenger::{create_action, Data, Res, Req};
///
/// create_action!(Main, |res: Res, req: Req| async move {
///     let my_data = Some(Data::new("my_value", None));
///     let quick_reply = QuickReply::new("Quick Reply Title", "http://url/image.jpg", Payload::new(MyAction, my_data));
///     let model = QuickReplyModel::new(&req.user, "Message Text", vec![quick_reply]);
///     res.send(model).await;
/// });
///
/// create_action!(MyAction, |res: Res, req: Req| async move {
///     todo!()
/// });
///
/// ```
///
/// In this example, a `QuickReply` is created with a title and a `Payload`. The `Payload` includes an `Action` and an optional `Data`.
/// Then, a `QuickReplyModel` is created with a sender ID, a message text, and a vector of `QuickReply`.
#[derive(Debug, Serialize)]
pub struct QuickReplyModel<'q> {
    recipient: Recipient<'q>,
    messaging_type: String,
    message: QuickMessage,
}

impl<'q> QuickReplyModel<'q> {
    pub fn new(sender: &'q str, message: &str, quick_replies: Vec<QuickReply>) -> Self {
        Self {
            recipient: Recipient { id: sender },
            messaging_type: "RESPONSE".into(),
            message: QuickMessage {
                text: message.into(),
                quick_replies,
            },
        }
    }
}

impl<'q> GetSender<'q> for QuickReplyModel<'q> {
    fn get_sender(&self) -> &'q str {
        self.recipient.id
    }
}

impl<'q> NextPrevNavigation<'q> for QuickReplyModel<'q> {}
