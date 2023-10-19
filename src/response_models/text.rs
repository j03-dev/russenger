use rocket::serde::Serialize;

use super::{recipient::Recipient, SendResponse};

#[derive(Serialize)]
pub struct Text<'t> {
    text: &'t str,
}

#[derive(Serialize)]
pub struct TextModel<'s> {
    recipient: Recipient<'s>,
    message: Text<'s>,
}

impl<'s> TextModel<'s> {
    pub fn new(sender: &'s str, text: &'s str) -> Self {
        Self {
            recipient: Recipient { id: sender },
            message: Text { text },
        }
    }
}

#[rocket::async_trait]
impl<'s> SendResponse for TextModel<'s> {}
