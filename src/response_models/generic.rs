use rocket::serde::Serialize;

pub use crate::Data;

use super::{GetSender, NextPrevNavigation, payload::Payload, recipient::Recipient};

/// Creates a new `GenericButton`.
///
/// # Arguments
///
/// * `title` - The title of the button.
/// * `payload` - The payload of the button.
///
/// # Example
///
/// ```rust
/// use russenger::{Res, Req, create_action};
/// use russenger::generic::GenericButton;
/// use russenger::payload::Payload;
///
/// create_action!(AnAction, |res: Res, req: Req| async move {
///     todo!()
/// });
///
/// let button = GenericButton::new("Button Title", Payload::new(AnAction, None));
/// ```
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

/// `GenericElement` represents an element in a generic template.
///
/// It has a title, image URL, subtitle, and a list of buttons.
#[derive(Debug, Serialize)]
pub struct GenericElement {
    title: String,
    image_url: String,
    subtitle: String,
    buttons: Vec<GenericButton>,
}

/// Creates a new `GenericElement`.
///
/// # Arguments
///
/// * `title` - The title of the element.
/// * `image_url` - The image URL of the element.
/// * `subtitle` - The subtitle of the element.
/// * `buttons` - The buttons of the element.
///
/// # Example
///
/// ```rust
/// use russenger::{Res, Req, create_action};
/// use russenger::generic::{GenericButton, GenericElement};
/// use russenger::payload::Payload;
///
/// create_action!(AnAction, |res: Res, req:Req| async move {
///     todo!()
/// });
///
/// let button = GenericButton::new("Button Title", Payload::new(AnAction, None));
/// let element = GenericElement::new("Element Title", "https://example.com/image.jpg", "Element Subtitle", vec![button]);
///
/// ```
///
impl GenericElement {
    pub fn new(
        title: &str,
        image_url: &str,
        subtitle: &str,
        buttons: Vec<GenericButton>,
    ) -> Self {
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

/// Creates a new `GenericModel`.
///
/// # Arguments
///
/// * `sender` - The sender of the message.
/// * `elements` - The elements of the generic template.
///
/// # Example
///
/// ```rust
/// use russenger::generic::{GenericButton, GenericElement, GenericModel};
/// use russenger::{create_action, Data, Req, Res};
/// use russenger::payload::Payload;
///
/// create_action!(Main, |res: Res, req: Req| async move {
///     let my_data = Some(Data::new("my_value", None));
///     let button = GenericButton::new("Button Title", Payload::new(MyAction, my_data));
///     let element = GenericElement::new("Element Title", "https://example.com/image.jpg", "Element Subtitle", vec![button]);
///     let model = GenericModel::new(&req.user, vec![element]);
///     res.send(model).await;
///; });
///
/// create_action!(MyAction, |res: Res, req: Req| async move {
///     todo!()
/// });
/// ```
///
/// In this example, a `GenericButton` is created with a title and a `Payload`. The `Payload` includes an `Action` and an optional `Data`.
/// Then, a `GenericElement` is created with a title, image URL, subtitle, and a vector of `GenericButton`.
/// Finally, a `GenericModel` is created with a sender ID and a vector of `GenericElement`.
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

impl<'g> NextPrevNavigation<'g> for GenericModel<'g> {
    // This struct implements the `NextPrevNavigation` trait but does not provide any additional methods.
}
