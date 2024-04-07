use rocket::serde::Serialize;

use super::recipient::Recipient;
use super::ResponseModel;

#[derive(Serialize)]
struct Text<'t> {
    text: &'t str,
}

#[derive(Serialize)]
pub struct TextModel<'s> {
    recipient: Recipient<'s>,
    messaging_type: &'s str,
    message: Text<'s>,
}

impl<'s> TextModel<'s> {
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
