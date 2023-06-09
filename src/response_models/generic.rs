use rocket::serde::Serialize;

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
            payload: "DEVELOPER_DEFINED_PAYLOAD",
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
pub struct Payload<'p> {
    pub template_type: String,
    pub elements: &'p Vec<GenericElement<'p>>,
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
    pub fn new(elements: &'m Vec<GenericElement<'m>>) -> Self {
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
    pub fn new(recipient: Recipient, message: GenericMessage<'g>) -> Self {
        Self { recipient, message }
    }
}
