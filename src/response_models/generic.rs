//! The `generic` module contains structs and methods for creating and sending generic template messages in Facebook Messenger.
//!
//! A generic template message is a type of structured message that can contain multiple `GenericElement`s.
//! Each `GenericElement` can contain a title, subtitle, image, and multiple buttons.
//!
//! This module provides the following structs:
//!
//! * `GenericElement`: Represents a single element in a generic template.
//! * `GenericModel`: Represents a generic template message.
//!
//! It also provides methods for creating and sending generic template messages.
//!
//! # Examples
//!
//! Creating and sending a generic template message:
//!
//! ```rust
//! use russenger::prelude::*;
//!
//! #[action]
//! async fn Main(res: Res, req: Req) {
//!     let elements = vec![
//!         GenericElement::new(
//!             "Title",
//!             "https://example.com/image.jpg",
//!             "Subtitle",
//!             vec![Button::Postback {
//!                 title: "Hello World".to_owned(),
//!                 payload: Payload::new(HelloWorld, Some(Data::new("Hello World!", None))),
//!             }],
//!         ),
//!         // More elements ....
//!     ];
//!
//!     let generic = GenericModel::new(&req.user, elements, None);
//!     res.send(generic).await;
//! }
//!
//! #[action]
//! async fn HelloWorld(res: Res, req: Req) {
//!     let hello_world: String = req.data.get_value();
//!     res.send(TextModel::new(&req.user, &hello_world)).await;
//! }
//! ```
//!
//! For more information on generic template messages, refer to the [Facebook Documentation](https://developers.facebook.com/docs/messenger-platform/send-messages/template/generic).
use core::panic;
use serde::Serialize;
use serde_json::value::Value;

use super::{
    button::Button,
    data::{Page, MAX_PAGE},
    recipient::Recipient,
    ResponseModel,
};

/// `GenericElement` is a struct that represents a single element in a generic template.
///
/// Each `GenericElement` can contain a title, subtitle, image, and multiple buttons.
///
/// # Fields
///
/// * `title`: A string that represents the title of the element.
/// * `image_url`: A string that represents the URL of the image to be displayed in the element.
/// * `subtitle`: A string that represents the subtitle of the element.
/// * `buttons`: A vector of `Button` structs that represent the buttons to be displayed in the element.
///
/// # Examples
///
/// Creating a `GenericElement`:
///
/// ```rust
/// // use russenger::response_models::data::Data;
/// // use russenger::response_models::button::Button;
/// // use russenger::response_models::payload::Payload;
/// // use russenger::response_models::generic::GenericElement;
///
/// use russenger::prelude::*; // if you use this import other imports are not needed;
///
/// let element = GenericElement::new(
///     "Title",
///     "https://example.com/image.jpg",
///     "Subtitle",
///     vec![Button::Postback {
///         title: "Hello World".to_owned(),
///         payload: Payload::new(HelloWorld, Some(Data::new("Hello World!", None))),
///     }],
/// );
///
///
/// #[action]
/// async fn HelloWorld(res: Res, req: Req) {
///     let hello_world: String = req.data.get_value();
///     res.send(TextModel::new(&req.user, &hello_world)).await;
/// }
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct GenericElement {
    title: String,
    image_url: String,
    subtitle: String,
    buttons: Vec<Value>,
}

impl GenericElement {
    /// `new` is a method of the `GenericElement` struct that creates a new instance of `GenericElement`.
    ///
    /// # Parameters
    ///
    /// * `title: &str` - The title of the element.
    /// * `image_url: &str` - The URL of the image to be displayed in the element.
    /// * `subtitle: &str` - The subtitle of the element.
    /// * `buttons: Vec<Button>` - A vector of `Button` structs that represent the buttons to be displayed in the element.
    ///
    /// # Panics
    ///
    /// This method will panic if the number of buttons is more than 3, as Facebook Messenger only allows a maximum of 3 buttons per element.
    ///
    /// # Returns
    ///
    /// A new `GenericElement` instance.
    ///
    /// # Examples
    ///
    /// Creating a new `GenericElement`:
    ///
    /// ```rust
    /// use russenger::prelude::*; // if you use this import other imports are not needed;
    ///
    /// let element = GenericElement::new(
    ///     "Title",
    ///     "https://example.com/image.jpg",
    ///     "Subtitle",
    ///     vec![Button::Postback {
    ///         title: "Hello World".to_owned(),
    ///         payload: Payload::new(HelloWorld, Some(Data::new("Hello World!", None))),
    ///     }],
    /// );
    ///
    /// #[action]
    /// async fn HelloWorld(res: Res, req: Req) {
    ///     let hello_world: String = req.data.get_value();
    ///     res.send(TextModel::new(&req.user, &hello_world)).await;
    /// }
    /// ```
    ///
    /// This example shows how to create a new `GenericElement`.
    pub fn new(title: &str, image_url: &str, subtitle: &str, buttons: Vec<Button>) -> Self {
        if buttons.len() > 3 {
            panic!("Buttons must be three maximum")
        }
        let buttons: Vec<_> = buttons.iter().map(|btn| btn.to_value()).collect();
        Self {
            title: title.into(),
            image_url: image_url.into(),
            subtitle: subtitle.into(),
            buttons,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct GenericPayload {
    pub template_type: String,
    pub elements: Vec<GenericElement>,
}

#[derive(Debug, Clone, Serialize)]
struct Attachment {
    #[serde(rename = "type")]
    pub r#type: String,
    pub payload: GenericPayload,
}

#[derive(Debug, Clone, Serialize)]
struct GenericMessage {
    pub attachment: Attachment,
}

/// `GenericModel` is a struct that represents a generic template message.
///
/// A generic template message is a type of structured message that can contain multiple `GenericElement`s.
/// Each `GenericElement` can contain a title, subtitle, image, and multiple buttons.
///
/// # Fields
///
/// * `recipient`: A `Recipient` struct that represents the recipient of the message.
/// * `messaging_type`: A string that represents the type of messaging. For generic template messages, this is always "RESPONSE".
/// * `message`: A `GenericMessage` struct that contains the `GenericElement`s to be displayed in the message.
///
/// # Methods
///
/// * `new(sender: &'g str, elements: Vec<GenericElement>, page: Option<Page>) -> Self` - Creates a new `GenericModel` instance.
/// * `get_sender() -> &'g str` - Returns the ID of the recipient.
/// * `is_element_empty() -> bool` - Returns whether the `GenericElement`s in the message are empty.
///
/// # Examples
///
/// Creating a `GenericModel` and sending it:
///
/// ```rust
/// // use russenger::response_models::data::Data;
/// // use russenger::response_models::button::Button;
/// // use russenger::response_models::payload::Payload;
/// // use russenger::response_models::generic::GenericElement;
///
/// use russenger::prelude::*; // if you use this import other imports are not needed;
///
/// #[action]
/// async fn Main(res: Res, req: Req) {
///     let elements = vec![
///         GenericElement::new(
///             "Title",
///             "https://example.com/image.jpg",
///             "Subtitle",
///             vec![Button::Postback {
///                 title: "Hello World".to_owned(),
///                 payload: Payload::new(HelloWorld, Some(Data::new("Hello World!", None))),
///             }],
///         ),
///         // More elements ....
///     ];
///
///     let message = GenericModel::new(&req.user, elements, None);
///     res.send(message).await;
/// }
///
/// #[action]
/// async fn HelloWorld(res: Res, req: Req) {
///     let hello_world: String = req.data.get_value();
///    res.send(TextModel::new(&req.user, &hello_world)).await;
/// }
/// ```
///
/// [Facebook Documentation](https://developers.facebook.com/docs/messenger-platform/send-messages/template/generic)
#[derive(Debug, Clone, Serialize)]
pub struct GenericModel<'g> {
    recipient: Recipient<'g>,
    messaging_type: &'g str,
    message: GenericMessage,
}

impl<'g> GenericModel<'g> {
    /// `new` is a method of the `GenericModel` struct that creates a new instance of `GenericModel`.
    ///
    /// # Parameters
    ///
    /// * `sender: &'g str` - The ID of the sender.
    /// * `elements: Vec<GenericElement>` - A vector of `GenericElement`s to be included in the message.
    /// * `page: Option<Page>` - An optional `Page` struct that represents the Facebook page that the message is being sent from.
    ///
    /// # Returns
    ///
    /// A new `GenericModel` instance.
    ///
    /// # Examples
    ///
    /// Creating a new `GenericModel`:
    ///
    /// ```rust
    /// use russenger::prelude::*; // if you use this import other imports are not needed;
    ///
    /// let elements = vec![
    ///     GenericElement::new(
    ///         "Title",
    ///         "https://example.com/image.jpg",
    ///         "Subtitle",
    ///         vec![Button::Postback {
    ///             title: "Hello World".to_owned(),
    ///             payload: Payload::new(HelloWorld, Some(Data::new("Hello World!", None))),
    ///         }],
    ///     ),
    ///     // More elements ....
    /// ];
    ///
    /// let message = GenericModel::new("sender_id", elements, None);
    ///
    /// #[action]
    /// async fn HelloWorld(res: Res, req: Req) {
    ///     let hello_world: String = req.data.get_value();
    ///    res.send(TextModel::new(&req.user, &hello_world)).await;
    /// }
    /// ```
    ///
    /// This example shows how to create a new `GenericModel` and send it.
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

impl ResponseModel for GenericModel<'_> {
    const END_POINT: &'static str = "messages";
}
