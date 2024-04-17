use rocket::serde::json::{json, Value};

use super::payload::Payload;

#[derive(Clone, Debug)]
pub enum Button<'b> {
    AccountUnlink,
    AccountLink { url: &'b str },
    WebUrl { title: &'b str, url: &'b str },
    Postback { title: &'b str, payload: Payload },
    PhoneNumber { title: &'b str, payload: Payload },
}

impl<'b> Button<'b> {
    pub fn to_value(&self) -> Value {
        match self.clone() {
            Self::AccountLink { url } => {
                json!({"type": "account_link", "url": url})
            }
            Self::AccountUnlink => {
                json!({"type": "account_unlink"})
            }
            Self::Postback { title, payload } | Self::PhoneNumber { title, payload } => json!({
                "type": "postback",
                "title": title,
                "payload": payload.to_string()
            }),
            Self::WebUrl { title, url } => json!({
                "type": "web_url",
                "title": title,
                "url": url
            }),
        }
    }
}
