use rocket::serde::Serialize;

use super::generic::Recipient;

#[derive(Serialize)]
pub struct QuickReplie<'c> {
    pub content_type: &'c str,
    pub title: &'c str,
    pub payload: &'c str,
    pub image_url: &'c str,
}

impl<'c> QuickReplie<'c> {
    pub fn new(title: &'c str, image_url: &'c str) -> Self {
        Self {
            content_type: "text",
            title,
            payload: "<POSTBACK_PAYLOAD>",
            image_url,
        }
    }
}

#[derive(Serialize)]
struct QuickMessage<'m> {
    pub text: &'m str,
    pub quick_replies: &'m Vec<QuickReplie<'m>>,
}

#[derive(Serialize)]
pub struct QuickReplieModel<'q> {
    pub recipient: Recipient<'q>,
    pub messaging_type: &'q str,
    message: QuickMessage<'q>,
}

impl<'q> QuickReplieModel<'q> {
    pub fn new(sender: &'q str, message: &'q str, quick_replies: &'q Vec<QuickReplie>) -> Self {
        Self {
            recipient: Recipient { id: sender },
            messaging_type: "RESPONSE",
            message: QuickMessage {
                text: message,
                quick_replies,
            },
        }
    }
}
