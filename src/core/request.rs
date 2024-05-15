use crate::query::Query;
use crate::response_models::data::Data;

/// The `Req` struct represents a request from a user.
///
/// It contains the following fields:
/// * `user`: A `String` that represents the user who made the request.
/// * `query`: A `Query` that represents the query made by the user.
/// * `data`: A `Data` that represents the data associated with the request.
/// * `host`: A `String` that represents the host from which the request was made.
#[derive(Clone)]
pub struct Req {
    pub user: String,

    /// The `Query` struct represents a database query.
    ///
    /// This struct is used to interact with the database. It contains a `db` field, which is an instance of the `DB` enum that represents the database connection.
    ///
    /// # Fields
    ///
    /// * `db`: The database connection. This is an instance of the `DB` enum.
    ///
    /// # Methods
    ///
    /// * `new`: This method creates a new `Query`. It establishes a connection to the database and returns a `Query` with the established connection.
    /// * `migrate`: This method creates a new table `russenger_user` in the database. It returns a boolean indicating whether the operation was successful.
    /// * `create`: This method inserts a new user into the `russenger_user` table. It takes a user ID as an argument and returns a boolean indicating whether the operation was successful.
    /// * `set_action`: This method updates the action of a user in the `russenger_user` table. It takes a user ID and an action as arguments and returns a boolean indicating whether the operation was successful.
    ///
    /// # Examples
    ///
    /// Creating a new `Query` and using it to insert a new user into the database:
    ///
    /// ```rust
    /// use russenger::prelude::*;
    ///
    /// #[action]
    /// async fn Main(res: Res, req: Req) {
    ///     req.query.set_action(&req.user, NextAction).await; // goto NextAction
    /// }
    /// 
    /// #[action]
    /// async fn NextAction(res: Res, req: Req) {}
    /// ```
    pub query: Query,

    /// The `Data` struct represents a data object with a value and an optional page.
    ///
    /// This struct is used to store and manipulate data. It contains a `value` field, which is a serialized JSON string, and a `page` field, which is an optional `Page` struct.
    ///
    /// # Fields
    ///
    /// * `value`: The value of the data. This is a serialized JSON string.
    /// * `page`: The page of the data. This is an optional `Page` struct.
    ///
    /// # Methods
    ///
    /// * `new`: This method creates a new `Data`. It takes a value and an optional page as arguments, serializes the value into a JSON string, and returns a `Data` with the serialized string and the page.
    /// * `get_value`: This method deserializes the value of the data into a specified type. It returns the deserialized value if it exists, or the default value of the type if it doesn't.
    /// * `get_page`: This method returns the page of the data.
    ///
    /// # Examples
    ///
    /// Creating a new `Data` and using it to get the value and the page:
    ///
    /// ```rust
    /// use russenger::response_models::data::{Data, Page};
    ///
    /// let data = Data::new("value", Some(Page::default()));
    /// let value: String = data.get_value();
    /// let page = data.get_page();
    /// ```
    pub data: Data,

    /// The `host` field represents the host name or IP address of the server that the request is being sent to.
    ///
    /// This field is used to specify the server that the request should be sent to. It is a `String` that contains the host name or IP address of the server.
    ///
    /// # Examples
    ///
    /// Creating a new `Req` and setting the `host` field:
    ///
    /// ```rust
    /// use russenger::prelude::*;
    ///
    /// #[action]
    /// async fn Main(res: Res, req: Req) {
    ///    let image_url = &format!("{host}/image.jpg", host = req.host);
    ///    let media = MediaModel::new(&req.user, "image", image_url);
    ///    res.send(media).await;
    /// }
    /// ```
    pub host: String,
}

impl Req {
    /// Creates a new `Req`.
    ///
    /// # Arguments
    ///
    /// * `user`: A string slice that holds the user.
    /// * `query`: A `Query` that holds the query.
    /// * `data`: A `Data` that holds the data.
    /// * `host`: A string slice that holds the host.
    ///
    /// # Returns
    ///
    /// A `Req` that contains the provided user, query, data, and host.
    pub fn new(user: &str, query: Query, data: Data, host: &str) -> Self {
        Self {
            user: user.to_owned(),
            query,
            data,
            host: host.to_owned(),
        }
    }
}
