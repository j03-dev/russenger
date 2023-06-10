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
pub struct QuickMessage<'m> {
    pub text: &'m str,
    pub quick_replies: &'m Vec<QuickReplie<'m>>,
}

impl<'m> QuickMessage<'m> {
    pub fn new(text: &'m str, quick_replies: &'m Vec<QuickReplie>) -> Self {
        Self {
            text,
            quick_replies,
        }
    }
}

#[derive(Serialize)]
pub struct QuickReplieModel<'q> {
    pub recipient: Recipient,
    pub messaging_type: &'q str,
    pub message: QuickMessage<'q>,
}

impl<'q> QuickReplieModel<'q> {
    pub fn new(sender: String, message: QuickMessage<'q>) -> Self {
        let recipient = Recipient { id: sender };
        Self {
            recipient,
            messaging_type: "RESPONSE",
            message,
        }
    }
}
