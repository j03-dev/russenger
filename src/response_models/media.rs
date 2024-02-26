use serde::Serialize;

use super::recipient::Recipient;

#[derive(Serialize)]
struct MediaPayload<'p> {
    url: &'p str,
    is_reusable: bool,
}

#[derive(Serialize)]
struct MediaAttachment<'a> {
    #[serde(rename = "type")]
    r#type: &'a str,
    payload: MediaPayload<'a>,
}

#[derive(Serialize)]
struct Attachment<'s> {
    attachment: MediaAttachment<'s>,
}

#[derive(Serialize)]
pub struct MediaModel<'m> {
    messaging_type: &'m str,
    recipient: Recipient<'m>,
    message: Attachment<'m>,
}

impl<'m> MediaModel<'m> {
    pub fn new(sender: &'m str, media_type: &'m str, url: &'m str) -> Self {
        Self {
            messaging_type: "RESPONSE",
            recipient: Recipient { id: sender },
            message: Attachment {
                attachment: MediaAttachment {
                    r#type: media_type,
                    payload: MediaPayload {
                        url,
                        is_reusable: true,
                    },
                },
            },
        }
    }
}
