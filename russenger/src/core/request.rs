//! The `request` module contains the `Req` struct, which represents a request from a user.
//!
//! The `Req` struct contains the following fields:
//! * `user`: A `String` that represents the user who made the request.
//! * `query`: A `Query` that represents the query made by the user.
//! * `data`: A `Data` that represents the data associated with the request.
//! * `host`: A `String` that represents the host from which the request was made.
//!
//! # Examples
//!
//! Use the `Req` to get the user and data from a request:
//!
//! ```rust
//! use russenger::prelude::*;
//!
//! async fn hello_world(res: Res, req: Req) -> Result<()> {
//!     let user: String = req.user;
//!     let message: String  = req.data.get_value()?;
//!     res.send(TextModel::new(&user, "Hello, world!")).await?;
//!
//!     Ok(())
//! }
//!```
//! Use the `Req` to get the user and query from a request:
//!
//! ```rust
//! use russenger::prelude::*;
//!
//! async fn index(res: Res, req: Req) -> Result<()> {
//!     res.send(TextModel::new(&req.user, "What is your name")).await?;
//!     res.redirect("/name").await?;
//!
//!     Ok(())
//! }
//!
//! async fn name(res: Res, req: Req)  -> Result<()> {
//!     let name: String = req.data.get_value()?;
//!     res.send(TextModel::new(&req.user, &format!("Hello, {}!", name))).await?;
//!
//!     Ok(())
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     App::init().await?
//!        .attach(router![("/", index),("/name", name)])
//!        .launch()
//!        .await?;
//!     Ok(())
//! }
//! ```
use std::sync::Arc;

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
    /// The user who made the request.
    ///
    /// This field is a `String` that represents the user who made the request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use russenger::prelude::*;
    ///
    /// async fn hello_world(res: Res, req: Req) -> Result<()> {
    ///     let user: String = req.user;
    ///     res.send(TextModel::new(&user, "Hello, world!")).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
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
    /// async fn index(res: Res, req: Req) -> Result<()> {
    ///     res.redirect("/next_action").await?; // goto NextAction
    ///
    ///     Ok(())
    /// }
    ///
    /// async fn next_action(res: Res, req: Req) -> Result<()> {
    ///     Ok(())
    /// }
    /// ```
    pub query: Arc<Query>,

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
    /// let data = Data::new_with_page("value", Some(Page::default()));
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
    /// async fn index(res: Res, req: Req) -> Result<()> {
    ///     let image_url = &format!("{host}/static/image.jpg", host = req.host);
    ///     let media = MediaModel::new(&req.user, "image", image_url);
    ///     res.send(media).await?;
    ///
    ///     Ok(())
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
    pub fn new(user: &str, query: Arc<Query>, data: Data, host: &str) -> Self {
        Self {
            user: user.to_owned(),
            query,
            data,
            host: host.to_owned(),
        }
    }

    /// Creates a new `Req` instance with updated data.
    ///
    /// This method allows you to create an updated version of the current `Req` object,
    /// substituting its `data` field. It can be useful for passing modified data to
    /// subsequent actions while retaining the existing request context (e.g., user information,
    /// session, etc.).
    ///
    /// # Example
    ///
    /// ```rust
    /// use russenger::prelude::*;
    ///
    /// async fn home(res: Res, req: Req) -> Result<()> {
    ///     let user_input: String = req.data.get_value()?; // Extract the current data
    ///     res.send(TextModel::new(&req.user, &format!("User input received: {}", user_input)))
    ///         .await?;
    ///
    ///     // Create a new `Req` instance with updated data
    ///     let updated_req = req.new_from(Data::new(user_input));
    ///
    ///     // Pass the updated request to the next action
    ///     next_action(res, updated_req).await?;
    ///     Ok(())
    /// }
    ///
    /// async fn next_action(res: Res, req: Req) -> Result<()> {
    ///     let user_input: String = req.data.get_value()?; // Retrieve the updated data
    ///     res.send(TextModel::new(&req.user, &format!("Processed user input: {}", user_input)))
    ///         .await?;
    ///     Ok(())
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     App::init().await?
    ///         .attach(router![
    ///             ("/", |res, req| async move {
    ///                     res.send(TextModel::new(&req.user, "Welcome!")).await?;
    ///                     res.redirect("/home").await?;
    ///                     Ok(())
    ///                 }
    ///             )
    ///         ])
    ///         .attach(router![("/home", home), ("/next_action", next_action)])
    ///         .launch()
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Parameters
    ///
    /// - `self`: The current `Req` instance.
    /// - `data`: The new `Data` instance that will replace the current `data` field.
    ///
    /// # Returns
    ///
    /// Returns a new `Req` instance with the specified `data` while preserving all other properties of the original `Req`.
    pub fn new_from(self, data: Data) -> Self {
        Self { data, ..self }
    }
}
