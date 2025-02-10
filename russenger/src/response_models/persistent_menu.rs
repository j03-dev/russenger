//! This module provides a `PersistentMenuModel` struct that represents a persistent menu in a Messenger conversation.
//!
//! ## PersistentMenuModel Struct
//!
//! The `PersistentMenuModel` struct represents a persistent menu in a Messenger conversation. The persistent menu is an always-on user interface element inside Messenger conversations. It's an easy way to help people discover and access the core functionality of your Messenger bot at any point in the conversation.
//!
//! ### Fields
//!
//! * `psid: String` - The ID of the recipient.
//! * `persistent_menu: Vec<Menu>` - The items in the persistent menu.
//!
//! ### Methods
//!
//! * `new(sender: &'p str, buttons: Vec<Button>) -> Self` - Creates a new `PersistentMenuModel` instance.
//!
//! ## Examples
//!
//! Creating a `PersistentMenuModel` and sending it:
//!
//! ```rust
//! use russenger::prelude::*;
//!
//! async fn index(res: Res, req: Req) -> Result<()> {
//!     // Need GetStart First Before Send PersistenceMenu
//!     res.send(GetStartedButtonModel::new(Payload::default())).await?;
//!     let buttons = vec![
//!         Button::Postback {
//!             title: "Option 1",
//!             payload: Payload::new("/option_1", None),
//!         },
//!         // More buttons
//!     ];
//!
//!     let menu = PersistentMenuModel::new(&req.user, buttons);
//!     res.send(menu).await?;
//!
//!     Ok(())
//! }
//!
//! async fn option_1(res: Res, req: Req) -> Result<()> {
//!     res.send(TextModel::new(&req.user, "Option_1")).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Returns
//!
//! A POST request to the Facebook API to send a persistent menu.
//!
//! ## Reference
//!
//! [Facebook Messenger Platform - Persistent Menu](https://developers.facebook.com/docs/messenger-platform/send-messages/persistent-menu)
use super::button::Button;
use super::ResponseModel;
use serde::Serialize;
use serde_json::value::Value;

#[derive(Serialize)]
struct Menu<'m> {
    locale: &'m str,
    composer_input_disabled: bool,
    call_to_actions: Vec<Value>,
}

/// `PersistentMenuModel` is a struct that represents a persistent menu in a Messenger conversation.
///
/// The persistent menu is an always-on user interface element inside Messenger conversations.
/// It's an easy way to help people discover and access the core functionality of your Messenger bot at any point in the conversation.
///
/// # Fields
///
/// * `psid`: A string that represents the ID of the recipient.
/// * `persistent_menu`: A vector of `Menu` structs that represent the items in the persistent menu.
///
/// # Methods
///
/// * `new(sender: &'p str, buttons: Vec<Button>) -> Self` - Creates a new `PersistentMenuModel` instance.
///
/// # Examples
///
/// Creating a `PersistentMenuModel` and sending it:
///
/// ```rust
/// use russenger::prelude::*;
///
/// async fn index(res: Res, req: Req) -> Result<()> {
///     // Need GetStart First Before Send PersistenceMenu
///     res.send(GetStartedButtonModel::new(Payload::default())).await?;
///     let buttons = vec![
///         Button::Postback {
///             title: "Option 1",
///             payload: Payload::new("/option_1", None),
///         },
///         // More buttons
///     ];
///
///     let menu = PersistentMenuModel::new(&req.user, buttons);
///     res.send(menu).await?;
///
///     Ok(())
/// }
///
///
/// async fn option_1(res: Res, req: Req) -> Result<()> {
///     res.send(TextModel::new(&req.user, "Option_1")).await?;
///
///     Ok(())
/// }
/// ```
///
/// [Facebook Documentation](https://developers.facebook.com/docs/messenger-platform/send-messages/persistent-menu)
#[derive(Serialize)]
pub struct PersistentMenuModel<'p> {
    psid: &'p str,
    persistent_menu: Vec<Menu<'p>>,
}

impl<'p> PersistentMenuModel<'p> {
    /// `new` is a method of the `PersistentMenuModel` struct that creates a new instance of `PersistentMenuModel`.
    ///
    /// # Parameters
    ///
    /// * `sender: &'p str` - The ID of the recipient.
    /// * `buttons: Vec<Button>` - A vector of `Button` structs that represent the buttons to be displayed in the persistent menu.
    ///
    /// # Returns
    ///
    /// A new `PersistentMenuModel` instance.
    ///
    /// # Examples
    ///
    /// Creating a new `PersistentMenuModel`:
    ///
    /// ```rust
    /// use russenger::prelude::*;
    ///
    /// let buttons = vec![
    ///     Button::Postback {
    ///         title: "Option 1",
    ///         payload: Payload::new("/option_1", None),
    ///     },
    ///     // More buttons
    /// ];
    ///
    /// let menu = PersistentMenuModel::new("sender_id", buttons);
    ///
    /// async fn option_1(res: Res, req: Req) -> Result<()> {
    ///     res.send(TextModel::new(&req.user, "Option_1")).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// This example shows how to create a new `PersistentMenuModel`.
    pub fn new(sender: &'p str, buttons: impl IntoIterator<Item = Button<impl ToString>>) -> Self {
        let buttons: Vec<_> = buttons.into_iter().map(|btn| btn.to_value()).collect();

        Self {
            psid: sender,
            persistent_menu: vec![Menu {
                locale: "default",
                composer_input_disabled: false,
                call_to_actions: buttons,
            }],
        }
    }
}

impl ResponseModel for PersistentMenuModel<'_> {
    const END_POINT: &'static str = "custom_user_settings";
}
