use rocket::serde::Serialize;

pub use crate::Data;

use super::{payload::Payload, recipient::Recipient, GetSender, NextPrevNavigation};

/// `GenericButton` represents a button in a generic template.
///
/// It has a type, title, and payload. The type is always "postback".
#[derive(Debug, Serialize)]
pub struct GenericButton {
    #[serde(rename = "type")]
    r#type: String,
    title: String,
    payload: String,
}

impl GenericButton {
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
    /// let button = GenericButton::new("Button Title", Payload::new(AnAction, None));
    /// ```
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

impl GenericElement {
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
    /// let button = GenericButton::new("Button Title", Payload::new(AnAction, None));
    /// let element = GenericElement::new("Element Title", "http://example.com/image.jpg", "Element Subtitle", vec![button]);
    /// ```
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

/// `GenericModel` represents a generic template message model.
///
/// It has a recipient and a message. The message is a `GenericMessage` which contains an `Attachment`.
#[derive(Debug, Serialize)]
pub struct GenericModel<'g> {
    recipient: Recipient<'g>,
    message: GenericMessage,
}

impl<'g> GenericModel<'g> {
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
    /// use russenger::Data;
    ///
    // let my_data = Some(Data::new("my_value", None));
    /// let button = GenericButton::new("Button Title", Payload::new(MyAction, my_data));
    /// let element = GenericElement::new("Element Title", "http://example.com/image.jpg", "Element Subtitle", vec![button]);
    /// let model = GenericModel::new("Sender ID", vec![element]);
    /// ```
    ///
    /// In this example, a `GenericButton` is created with a title and a `Payload`. The `Payload` includes an `Action` and an optional `Data`.
    /// Then, a `GenericElement` is created with a title, image URL, subtitle, and a vector of `GenericButton`.
    /// Finally, a `GenericModel` is created with a sender ID and a vector of `GenericElement`.
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
    /// Returns the sender of the message.
    ///
    /// # Example
    ///
    /// ```rust
    /// let sender = model.get_sender();
    /// ```
    fn get_sender(&self) -> &'g str {
        self.recipient.id
    }
}

impl<'g> NextPrevNavigation<'g> for GenericModel<'g> {
    // This struct implements the `NextPrevNavigation` trait but does not provide any additional methods.
}
