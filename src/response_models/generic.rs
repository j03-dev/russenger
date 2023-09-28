use rocket::serde::Serialize;

use super::SendResponse;

#[derive(Serialize)]
pub struct GenericButton<'b> {
    #[serde(rename = "type")]
    pub r#type: &'b str,
    pub title: &'b str,
    pub payload: &'b str,
}

impl<'b> GenericButton<'b> {
    pub fn new(title: &'b str) -> Self {
        Self {
            r#type: "postback",
            title,
            payload: "<POSTBACK_PAYLOAD>",
        }
    }
}

#[derive(Serialize)]
pub struct GenericElement<'e> {
    pub title: &'e str,
    pub image_url: &'e str,
    pub subtitle: &'e str,
    pub buttons: Vec<GenericButton<'e>>,
}

#[derive(Serialize)]
struct Payload<'p> {
    pub template_type: &'p str,
    pub elements: &'p Vec<GenericElement<'p>>,
}

#[derive(Serialize)]
struct Attachment<'a> {
    #[serde(rename = "type")]
    pub r#type: &'a str,
    pub payload: Payload<'a>,
}

#[derive(Serialize)]
struct GenericMessage<'m> {
    pub attachment: Attachment<'m>,
}

#[derive(Serialize)]
pub struct Recipient<'r> {
    pub id: &'r str,
}

#[derive(Serialize)]
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
                    payload: Payload {
                        template_type: "generic",
                        elements,
                    },
                },
            },
        }
    }
}

#[rocket::async_trait]
impl<'g> SendResponse for GenericModel<'g> {}
