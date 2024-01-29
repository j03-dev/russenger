use rocket::serde::Serialize;

pub use crate::Data;

use super::{payload::Payload, recipient::Recipient, GetSender, NextPrevNavigation};

#[derive(Debug, Serialize)]
pub struct GenericButton<'b> {
    #[serde(rename = "type")]
    r#type: &'b str,
    title: String,
    payload: String,
}

impl<'b> GenericButton<'b> {
    pub fn new(title: &str, payload: Payload) -> Self {
        Self {
            r#type: "postback",
            title: title.into(),
            payload: payload.to_uri_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GenericElement<'e> {
    title: &'e str,
    image_url: &'e str,
    subtitle: &'e str,
    buttons: Vec<GenericButton<'e>>,
}

impl<'e> GenericElement<'e> {
    pub fn new(
        title: &'e str,
        image_url: &'e str,
        subtitle: &'e str,
        buttons: Vec<GenericButton<'e>>,
    ) -> Self {
        Self {
            title,
            image_url,
            subtitle,
            buttons,
        }
    }
}

#[derive(Debug, Serialize)]
struct GenericPayload<'p> {
    pub template_type: &'p str,
    pub elements: &'p Vec<GenericElement<'p>>,
}

#[derive(Debug, Serialize)]
struct Attachment<'a> {
    #[serde(rename = "type")]
    pub r#type: &'a str,
    pub payload: GenericPayload<'a>,
}

#[derive(Debug, Serialize)]
struct GenericMessage<'m> {
    pub attachment: Attachment<'m>,
}

#[derive(Debug, Serialize)]
pub struct GenericModel<'g> {
    recipient: Recipient<'g>,
    message: GenericMessage<'g>,
}

impl<'g> GenericModel<'g> {
    pub fn new(sender: &'g str, elements: &'g Vec<GenericElement>) -> Self {
        Self {
            recipient: Recipient { id: sender },
            message: GenericMessage {
                attachment: Attachment {
                    r#type: "template",
                    payload: GenericPayload {
                        template_type: "generic",
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
