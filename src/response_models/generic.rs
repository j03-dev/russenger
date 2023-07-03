use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct GenericButton {
    #[serde(rename = "type")]
    pub r#type: String,
    pub title: String,
    pub payload: String,
}

impl GenericButton {
    pub fn new(title: String) -> Self {
        Self {
            r#type: "postback".to_string(),
            title,
            payload: "DEVELOPER_DEFINED_PAYLOAD".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct GenericElement {
    pub title: String,
    pub image_url: String,
    pub subtitle: String,
    pub buttons: Vec<GenericButton>,
}

#[derive(Serialize)]
pub struct Payload<'p> {
    pub template_type: String,
    pub elements: &'p Vec<GenericElement>,
}

#[derive(Serialize)]
pub struct Attachment<'a> {
    #[serde(rename = "type")]
    pub r#type: &'a str,
    pub payload: Payload<'a>,
}

#[derive(Serialize)]
pub struct GenericMessage<'m> {
    pub attachment: Attachment<'m>,
}

impl<'m> GenericMessage<'m> {
    pub fn new(elements: &'m Vec<GenericElement>) -> Self {
        Self {
            attachment: Attachment {
                r#type: "template",
                payload: Payload {
                    template_type: "generic".to_string(),
                    elements,
                },
            },
        }
    }
}

#[derive(Serialize)]
pub struct Recipient {
    pub id: String,
}

#[derive(Serialize)]
pub struct GenericModel<'g> {
    pub recipient: Recipient,
    pub message: GenericMessage<'g>,
}

impl<'g> GenericModel<'g> {
    pub fn new(sender: String, message: GenericMessage<'g>) -> Self {
        let recipient = Recipient { id: sender };
        Self { recipient, message }
    }
}
