use rocket::serde::json::Value;
use rocket::serde::Serialize;

use crate::{
    core::{
        data::{Page, MAX_PAGE},
        response::Res as res,
    },
    Action, Data,
};

use super::{
    button::Button,
    payload::Payload,
    quick_replies::{QuickReply, QuickReplyModel},
    recipient::Recipient,
};

#[derive(Debug, Clone, Serialize)]
pub struct GenericElement {
    title: String,
    image_url: String,
    subtitle: String,
    buttons: Vec<Value>,
}

impl GenericElement {
    pub fn new(title: &str, image_url: &str, subtitle: &str, buttons: Vec<Button>) -> Self {
        let buttons: Vec<_> = buttons.iter().map(|btn| btn.to_value()).collect();
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
    messaging_type: &'g str,
    message: GenericMessage,
}

impl<'g> GenericModel<'g> {
    pub fn new(sender: &'g str, mut elements: Vec<GenericElement>, page: Option<Page>) -> Self {
        if let Some(p) = page {
            elements = elements.into_iter().skip(p.0).take(p.1 - p.0).collect();
        } else if elements.len() >= MAX_PAGE {
            elements.truncate(MAX_PAGE);
        }
        Self {
            recipient: Recipient { id: sender },
            messaging_type: "RESPONSE",
            message: GenericMessage {
                attachment: Attachment {
                    r#type: "template".to_owned(),
                    payload: GenericPayload {
                        template_type: "generic".to_owned(),
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

    pub async fn send_next<A: Action>(&self, action: A, data: Data) {
        let quick_reply = if !self.is_element_empty() {
            let mut page = data.get_page().unwrap_or_default();
            page.next();
            let value: String = data.get_value();
            QuickReply::new(
                "Next",
                "",
                Payload::new(action, Some(Data::new(value, Some(page)))),
            )
        } else {
            QuickReply::new("Back", "", Payload::default())
        };
        res.send(QuickReplyModel::new(
            self.get_sender(),
            "Navigation",
            vec![quick_reply],
        ))
        .await;
    }
}
