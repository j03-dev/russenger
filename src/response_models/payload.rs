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
//! use russenger::prelude::*;
//! 
//! #[action]
//! async fn Main(res: Res, req: Req) {
//!     let data = Data::new("HelloWorld", None);
//!     let payload = Payload::new(HelloWorld, Some(data));
//!     res.send(GetStartedModel::new(payload)).await;   
//! }
//!
//! #[action]
//! async fn HelloWorld(res: Res, req: Req) {
//!    let value: String = req.data.get_value();
//!    res.send(TextModel::new(&req.user, &value)).await;
//! }
//! 
//! russenger_app!(Main, HelloWorld);
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
use crate::Action;

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
/// let payload = Payload::new(HelloWorld, Some(data));
///
/// #[action]
/// async fn HelloWorld(res: Res, req: Req) {
///    let value: String = req.data.get_value();
///    res.send(TextModel::new(&req.user, &value)).await;
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
    path: String,
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
    /// let payload = Payload::new(SomeAction, None);
    ///
    /// #[action]
    /// async fn SomeAction(res: Res, req: Req) {
    ///    res.send(TextModel::new(&req.user, "SomeAction")).await;
    /// }
    ///
    /// ```
    pub fn new<A: Action>(action: A, data: Option<Data>) -> Self {
        Self {
            path: action.path(),
            data,
        }
    }

    /// Creates a new `Payload` instance with a specified path and optional data.
    ///
    /// # Parameters
    ///
    /// * `path: String` - The path associated with the payload.
    /// * `data: Option<Data>` - Optional data to be included in the payload.
    ///
    /// # Returns
    ///
    /// A new `Payload` instance with the specified path and data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use russenger::prelude::*;
    /// 
    /// let data = Some(Data::new("key", None));
    /// let payload = Payload::new_with_path("Main".to_string(), data);
    /// ```
    pub fn new_with_path(path: String, data: Option<Data>) -> Self {
        Self { path, data }
    }

    /// Returns the path associated with the payload.
    ///
    /// # Returns
    ///
    /// The path associated with the payload.
    pub fn get_path(&self) -> String {
        self.path.clone()
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
            Err(_) => write!(f, "") // Outputs an empty string if there is an error in serialization
        }
    }
}

impl Default for Payload {
    fn default() -> Self {
        Payload {
            path: "Main".to_owned(),
            data: None,
        }
    }
}
