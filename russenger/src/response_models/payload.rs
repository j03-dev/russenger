//! This module provides a `Payload` struct that represents the payload of a request in a Messenger conversation.
//!
//! ## Payload Struct
//!
//! The `Payload` struct represents the payload of a request in a Messenger conversation. The payload contains the path of the action to be performed and optional data associated with the action.
//!
//! ### Fields
//!
//! * `path: String` - The path of the action to be performed.
//! * `data: Option<Data>` - The data associated with the action. This field is optional.
//!
//! ### Methods
//!
//! * `new<A: Action>(action: A, data: Option<Data>) -> Self` - Creates a new `Payload` instance. The `action` parameter is the action to be performed, and the `data` parameter is the data associated with the action.
//! * `get_data(&self) -> Data` - Returns the data associated with the action. If there is no data, it returns the default value of `Data`.
//!
//! ## Examples
//!
//! Creating a `Payload` and getting its path and data:
//!
//! ```rust
//! use russenger::models::RussengerUser;
//! use russenger::prelude::*;
//!
//! #[action]
//! async fn index (res: Res, req: Req) -> Result<()> {
//!     let data = Data::new("HelloWorld", None);
//!     let payload = Payload::new("/hello_world", Some(data));
//!     res.send(GetStartedButtonModel::new(payload)).await?;
//!
//!     Ok(())
//! }
//!
//! #[action]
//! async fn hello_world(res: Res, req: Req) -> Result<()> {
//!    let value: String = req.data.get_value();
//!    res.send(TextModel::new(&req.user, &value)).await?;
//!
//!    Ok(())
//! }
//!
//! #[russenger::main]
//! async fn main() -> Result<()> {
//!     let conn = Database::new().await?.conn;
//!     migrate!([RussengerUser], &conn);
//!     let mut app = App::init().await?;
//!     app.add("/", index).await;
//!     app.add("/hello_world", hello_world).await;
//!     launch(app).await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Implements
//!
//! * `FromStr`
//! * `ToString`
//! * `Default`
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::data::Data;
/// `Payload` is a struct that represents the payload of a request in a Messenger conversation.
///
/// The payload contains the path of the action to be performed and optional data associated with the action.
///
/// # Fields
///
/// * `path: String` - The path of the action to be performed.
/// * `data: Option<Data>` - The data associated with the action. This field is optional.
///
/// # Methods
///
/// * `new<A: Action>(action: A, data: Option<Data>) -> Self` - Creates a new `Payload` instance.
///   The `action` parameter is the action to be performed, and the `data` parameter is the data associated with the action.
/// * `get_data(&self) -> Data` - Returns the data associated with the action. If there is no data, it returns the default value of `Data`.
///
/// # Examples
///
/// Creating a `Payload` and getting its path and data:
///
/// ```rust
/// use russenger::prelude::*;
///
/// let data = Data::new("HelloWorld", None);
/// let payload = Payload::new("/hello_world", Some(data));
///
/// #[action]
/// async fn hello_world(res: Res, req: Req) -> Result<()> {
///    let value: String = req.data.get_value();
///    res.send(TextModel::new(&req.user, &value)).await?;
///
///    Ok(())
/// }
/// ```
///
/// # Implements
///
/// * `FromStr`
/// * `ToString`
/// * `Default`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    action_path: String,
    data: Option<Data>,
}

impl Payload {
    /// Creates a new `Payload` instance.
    ///
    /// # Parameters
    ///
    /// * `action: A` - The action to be performed.
    /// * `data: Option<Data>` - The data associated with the action.
    ///
    /// # Returns
    ///
    /// A new `Payload` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use russenger::prelude::*;
    ///
    /// let payload = Payload::new("/some_action", None);
    ///
    /// #[action]
    /// async fn some_action(res: Res, req: Req) -> Result<()> {
    ///    res.send(TextModel::new(&req.user, "SomeAction")).await?;
    ///
    ///    Ok(())
    /// }
    ///
    /// ```
    pub fn new(action_path: &str, data: Option<Data>) -> Self {
        Self {
            action_path: action_path.to_owned(),
            data,
        }
    }

    /// Returns the path associated with the payload.
    ///
    /// # Returns
    ///
    /// The path associated with the payload.
    pub fn get_path(&self) -> String {
        self.action_path.clone()
    }

    /// Returns the data associated with the payload.
    ///
    /// # Returns
    ///
    /// The data associated with the payload.
    pub fn get_data(&self) -> Data {
        self.data.clone().unwrap_or_default()
    }
}

impl FromStr for Payload {
    type Err = String;

    fn from_str(payload: &str) -> Result<Self, String> {
        serde_json::from_str(payload).map_err(|err| err.to_string())
    }
}

impl std::fmt::Display for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json_string) => write!(f, "{}", json_string),
            Err(_) => write!(f, ""), // Outputs an empty string if there is an error in serialization
        }
    }
}

impl Default for Payload {
    fn default() -> Self {
        Payload {
            action_path: "/".to_owned(),
            data: None,
        }
    }
}
