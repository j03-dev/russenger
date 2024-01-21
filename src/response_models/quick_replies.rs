use rocket::serde::Serialize;

use super::{payload::Payload, recipient::Recipient};
use super::{GetSender, NextPrevNavigation};

#[derive(Serialize)]
pub struct QuickReply<'r> {
    content_type: &'r str,
    title: String,
    payload: String,
    image_url: String,
}

impl<'r> QuickReply<'r> {
    pub fn new(title: &str, image_url: &str, payload: Payload) -> Self {
        Self {
            content_type: "text",
            title: title.into(),
            payload: payload.to_uri_string(),
            image_url: image_url.into(),
        }
    }
}

#[derive(Serialize)]
struct QuickMessage<'m> {
    text: String,
    quick_replies: &'m Vec<QuickReply<'m>>,
}

#[derive(Serialize)]
pub struct QuickReplyModel<'q> {
    recipient: Recipient<'q>,
    messaging_type: String,
    message: QuickMessage<'q>,
}

impl<'q> QuickReplyModel<'q> {
    pub fn new(sender: &'q str, message: &str, quick_replies: &'q Vec<QuickReply>) -> Self {
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
