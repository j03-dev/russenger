use rocket::serde::Serialize;

use super::generic::Recipient;

#[derive(Serialize)]
pub struct Text<'t> {
    pub text: &'t str,
}

#[derive(Serialize)]
pub struct TextModel<'s> {
    pub recipient: Recipient,
    pub message: Text<'s>,
}
