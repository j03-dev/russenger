use rocket::serde::Serialize;

use super::generic::Recipient;

#[derive(Serialize)]
pub struct Text<'t> {
    pub text: &'t str,
}

#[derive(Serialize)]
pub struct TextModel<'s> {
    pub recipient: Recipient<'s>,
    pub message: Text<'s>,
}

impl<'s> TextModel<'s> {
    pub fn new(sender: &'s str, text: &'s str) -> Self {
        Self {
            recipient: Recipient { id: sender },
            message: Text { text },
        }
    }
}
