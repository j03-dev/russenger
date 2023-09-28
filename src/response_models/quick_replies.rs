use rocket::serde::Serialize;

use super::{generic::Recipient, SendResponse};

#[derive(Serialize)]
pub struct QuickReplie<'r> {
    pub content_type: &'r str,
    pub title: String,
    pub payload: &'r str,
    pub image_url: String,
}

impl QuickReplie<'r> {
    pub fn new(title: &str, image_url: &str) -> Self {
        Self {
            content_type: "text",
            title: title.into(),
            payload: "<POSTBACK_PAYLOAD>",
            image_url: image_url.into(),
        }
    }
}

#[derive(Serialize)]
struct QuickMessage<'m> {
    pub text: String,
    pub quick_replies: &'m Vec<QuickReplie<'m>>,
}

#[derive(Serialize)]
pub struct QuickReplieModel<'q> {
    pub recipient: Recipient<'q>,
    pub messaging_type: String,
    message: QuickMessage<'q>,
}

impl<'q> QuickReplieModel<'q> {
    pub fn new(sender: &'q str, message: &str, quick_replies:  &'q Vec<QuickReplie>) -> Self {
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
