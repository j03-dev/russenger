use rocket::serde::Serialize;

pub use crate::Data;

use super::{payload::Payload, recipient::Recipient, GetSender, NextPrevNavigation};

#[derive(Debug, Serialize)]
pub struct GenericButton {
    #[serde(rename = "type")]
    r#type: String,
    title: String,
    payload: String,
}

impl GenericButton {
    pub fn new(title: &str, payload: Payload) -> Self {
        Self {
            r#type: "postback".into(),
            title: title.into(),
            payload: payload.to_uri_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GenericElement {
    title: String,
    image_url: String,
    subtitle: String,
    buttons: Vec<GenericButton>,
}

impl GenericElement {
    pub fn new(title: &str, image_url: &str, subtitle: &str, buttons: Vec<GenericButton>) -> Self {
        Self {
            title: title.into(),
            image_url: image_url.into(),
            subtitle: subtitle.into(),
            buttons,
        }
    }
}

#[derive(Debug, Serialize)]
struct GenericPayload {
    pub template_type: String,
    pub elements: Vec<GenericElement>,
}

#[derive(Debug, Serialize)]
struct Attachment {
    #[serde(rename = "type")]
    pub r#type: String,
    pub payload: GenericPayload,
}

#[derive(Debug, Serialize)]
struct GenericMessage {
    pub attachment: Attachment,
}

#[derive(Debug, Serialize)]
pub struct GenericModel<'g> {
    recipient: Recipient<'g>,
    message: GenericMessage,
}

impl<'g> GenericModel<'g> {
    pub fn new(sender: &'g str, elements: Vec<GenericElement>) -> Self {
        Self {
            recipient: Recipient { id: sender },
            message: GenericMessage {
                attachment: Attachment {
                    r#type: "template".into(),
                    payload: GenericPayload {
                        template_type: "generic".into(),
                        elements,
                    },
                },
            },
        }
    }
}

impl<'g> GetSender<'g> for GenericModel<'g> {
    fn get_sender(&self) -> &'g str {
        self.recipient.id
    }
}

impl<'g> NextPrevNavigation<'g> for GenericModel<'g> {}
