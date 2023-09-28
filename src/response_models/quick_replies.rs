use rocket::serde::Serialize;

use super::{generic::Recipient, SendResponse};

#[derive(Serialize)]
pub struct QuickReplie {
    pub content_type: String,
    pub title: String,
    pub payload: String,
    pub image_url: String,
}

impl QuickReplie {
    pub fn new(title: String, image_url: String) -> Self {
        Self {
            content_type: "text".into(),
            title,
            payload: "<POSTBACK_PAYLOAD>".into(),
            image_url,
        }
    }
}

#[derive(Serialize)]
struct QuickMessage<'m> {
    pub text: String,
    pub quick_replies: &'m Vec<QuickReplie>,
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
