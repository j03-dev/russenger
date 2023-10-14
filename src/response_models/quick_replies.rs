use rocket::serde::Serialize;

use super::{generic::Recipient, payload::Payload, SendResponse};

#[derive(Serialize)]
pub struct QuickReplie<'r> {
    content_type: &'r str,
    title: String,
    payload: String,
    image_url: String,
}

impl<'r> QuickReplie<'r> {
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
    quick_replies: &'m Vec<QuickReplie<'m>>,
}

#[derive(Serialize)]
pub struct QuickReplieModel<'q> {
    recipient: Recipient<'q>,
    messaging_type: String,
    message: QuickMessage<'q>,
}

impl<'q> QuickReplieModel<'q> {
    pub fn new(sender: &'q str, message: &str, quick_replies: &'q Vec<QuickReplie>) -> Self {
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

#[rocket::async_trait]
impl<'q> SendResponse for QuickReplieModel<'q> {}
