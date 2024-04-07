use rocket::serde::Serialize;

use super::ResponseModel;
use super::{payload::Payload, recipient::Recipient};

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

#[derive(Debug, Serialize)]
pub struct QuickReplyModel<'q> {
    recipient: Recipient<'q>,
    messaging_type: &'q str,
    message: QuickMessage,
}

impl<'q> QuickReplyModel<'q> {
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
