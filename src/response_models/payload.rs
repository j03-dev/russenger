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

    pub fn new_with_path(path: String, data: Option<Data>) -> Self {
        Self { path, data }
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

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

impl ToString for Payload {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
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
