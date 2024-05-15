use serde::Serialize;

use super::{payload::Payload, ResponseModel};

/// `GetStartedModel` is a struct that represents the GET STARTED button in a Messenger conversation.
///
/// This button is shown when the user talks to the bot for the first time.
///
/// # Fields
///
/// * `get_started: String` - The payload of the GET STARTED button.
///
/// # Methods
///
/// * `new(payload: Payload) -> Self` - Creates a new `GetStartedModel` instance.
///
/// # Examples
///
/// Creating a `GetStartedModel` and sending it:
///
/// ```rust
/// use russenger::prelude::*;
///
/// #[action]
/// async fn Main(res: Res, req: Req) {
///     res.send(GetStartedModel::new(Payload::default())).await;
/// }
/// ```
///
/// # Returns
///
/// A POST request to the Facebook API to send a media file using the Facebook URL.
///
/// # Reference
///
/// [Facebook Messenger Platform - Get Started Button](https://developers.facebook.com/docs/messenger-platform/reference/messenger-profile-api/get-started-button)
#[derive(Debug, Clone, Serialize)]
pub struct GetStartedModel {
    get_started: String,
}

impl GetStartedModel {
    /// Creates a new `GetStartedModel` instance.
    ///
    /// # Parameters
    ///
    /// * `payload: Payload` - The payload of the GET STARTED button.
    ///
    /// # Returns
    ///
    /// A new `GetStartedModel` instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use russenger::response_models::get_started::GetStartedModel;
    /// use russenger::response_models::payload::Payload;
    ///
    /// GetStartedModel::new(Payload::default());
    /// ```
    pub fn new(payload: Payload) -> Self {
        Self {
            get_started: payload.to_string(),
        }
    }
}

impl ResponseModel for GetStartedModel {
    const END_POINT: &'static str = "messenger_profile";
}
