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
            payload: "<POSTBACK_PAYLOAD>".to_string(),
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
struct Payload<'p> {
    pub template_type: String,
    pub elements: &'p Vec<GenericElement>,
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
    pub recipient: Recipient<'g>,
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
                        template_type: "generic".into(),
                        elements,
                    },
                },
            },
        }
    }
}
