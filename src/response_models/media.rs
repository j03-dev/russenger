use rocket::serde::Serialize;

use super::{generic::Recipient, SendResponse};

#[derive(Serialize)]
struct MediaPayload<'p> {
    url: &'p str,
    is_resuable: bool,
}

#[derive(Serialize)]
struct MediaAttachment<'a> {
    #[serde(rename = "type")]
    r#type: &'a str,
    payload: MediaPayload<'a>,
}

#[derive(Serialize)]
pub struct MediaModel<'m> {
    recipient: Recipient<'m>,
    message: MediaAttachment<'m>,
}

impl<'m> MediaModel<'m> {
    pub fn new(sender: &'m str, media_type: &'m str, image_url: &'m str) -> Self {
        Self {
            recipient: Recipient { id: sender },
            message: MediaAttachment {
                r#type: media_type,
                payload: MediaPayload {
                    url: image_url,
                    is_resuable: true,
                },
            },
        }
    }
}

#[rocket::async_trait]
impl<'m> SendResponse for MediaModel<'m> {}
