use rocket::serde::Serialize;

use super::generic::Recipient;

#[derive(Serialize)]
pub struct Text<'t> {
    pub text: &'t str,
}

#[derive(Serialize)]
pub struct SimpleModel<'s> {
    pub recipient: Recipient,
    pub message: Text<'s>,
}
