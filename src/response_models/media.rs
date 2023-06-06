use rocket::serde::Serialize;

use super::generic::Recipient;

#[derive(Serialize)]
pub struct MediaPayload<'p> {
    url: &'p str,
    is_resuable: bool
}

impl<'p> MediaPayload<'p> {
    pub fn new(url: &'p str) -> Self {
        Self { url, is_resuable: true }
    }
}


#[derive(Serialize)]
pub struct MediaAttachment<'a> {
    #[serde(rename = "type")]
    r#type: &'a str,
    payload: MediaPayload<'a> 
}

impl<'a> MediaAttachment<'a> {
    pub fn new(r#type: &'a str, url: &'a str) -> Self {
        MediaAttachment {
            r#type,
            payload: MediaPayload::new(url)
        }
    }
}


#[derive(Serialize)]
pub struct MediaModel<'m> {
    recipient: Recipient,
    message: MediaAttachment<'m>
}

impl<'m> MediaModel<'m> {
    pub fn new(recipient: Recipient, message: MediaAttachment<'m>) -> Self {
        Self {
            recipient,
            message,
        }
    }
}