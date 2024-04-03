use rocket::serde::json::{json, Value};

use crate::payload::Payload;

#[derive(Clone, Debug)]
pub enum Button<'gb> {
    Postback { title: &'gb str, payload: Payload },
    WebUrl { title: &'gb str, url: String },
}

impl<'gb> Button<'gb> {
    pub fn to_value(&self) -> Value {
        match self.clone() {
            Self::Postback { title, payload } => json!({
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
