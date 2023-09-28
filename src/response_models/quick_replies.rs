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
    pub fn new(title: &str, image_url: &str) -> Self {
        Self {
            content_type: "text".into(),
            title: title.into(),
            payload: "<POSTBACK_PAYLOAD>".into(),
            image_url: image_url.into(),
        }
    }
}

#[derive(Serialize)]
struct QuickMessage {
    pub text: String,
    pub quick_replies: Vec<QuickReplie>,
}

#[derive(Serialize)]
pub struct QuickReplieModel<'q> {
    pub recipient: Recipient<'q>,
    pub messaging_type: String,
    message: QuickMessage,
}

impl<'q> QuickReplieModel<'q> {
    pub fn new(sender: &'q str, message: &str, quick_replies:  Vec<QuickReplie>) -> Self {
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
