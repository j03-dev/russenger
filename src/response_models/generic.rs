use rocket::serde::Serialize;

use crate::{
    core::{data::Pagination, response::Res as res},
    quick_replies::QuickReply,
    Action, Data,
};

use super::{payload::Payload, quick_replies::QuickReplyModel, recipient::Recipient};

const MAX_ELEMENT: usize = 10;

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Serialize)]
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
    pub fn new(sender: &'g str, mut elements: Vec<GenericElement>, pages: Pagination) -> Self {
        if let Some([start, end]) = pages {
            elements = elements.into_iter().skip(start).take(end - start).collect();
        } else if elements.len() >= MAX_ELEMENT {
            elements.truncate(MAX_ELEMENT);
        }
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

impl<'g> GenericModel<'g> {
    fn get_sender(&self) -> &'g str {
        self.recipient.id
    }

    fn is_element_empty(&self) -> bool {
        self.message.attachment.payload.elements.is_empty()
    }

    pub async fn send_next_prev<A: Action>(&self, action: A, data: Data) {
        let mut navigations: Vec<QuickReply> = Vec::new();
        if !self.is_element_empty() {
            let [start, end] = data.get_page().unwrap_or([0, MAX_ELEMENT]);
            let value: String = data.get_value();
            if start & end >= MAX_ELEMENT {
                navigations.push(QuickReply::new(
                    "Prev",
                    "",
                    Payload {
                        path: action.path(),
                        data: Some(Data::new(
                            &value,
                            Some([start - MAX_ELEMENT, end - MAX_ELEMENT]),
                        )),
                    },
                ));
            }

            navigations.push(QuickReply::new(
                "Next",
                "",
                Payload {
                    path: action.path(),
                    data: Some(Data::new(
                        &value,
                        Some([start + MAX_ELEMENT, end + MAX_ELEMENT]),
                    )),
                },
            ));
        } else {
            navigations.push(QuickReply::new("Back", "", Payload::default()));
        }
        res.send(QuickReplyModel::new(
            self.get_sender(),
            "Navigation",
            navigations,
        ))
        .await;
    }
}
