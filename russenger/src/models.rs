//! The `models` module contains the data models used by the Russenger library.
//!
//! ## Structs
//!
//! ### `RussengerUser`
//!
//! The `RussengerUser` struct represents a user in the Russenger bot framework. It has two fields:
//!
//! - `facebook_user_id`: A string that represents the user's Facebook user ID. This field is the primary key of the table.
//! - `action`: A string that represents the current action of the user. This field has a default value of "Main".
//!
//! The `RussengerUser` struct implements the `FromRow` and `Model` traits from the `rusql_alchemy` crate, which allows it to be used with the `rusql_alchemy` ORM.
//!
//! ## Query Module
//!
//! The `query` module provides utilities for handling queries. The `Query` struct represents a database query. It includes a `DB` enum that represents the database connection.
//!
//! ### Methods
//!
//! - `new`: This method creates a new `Query`. It establishes a connection to the database and returns a `Query` with the established connection.
//! - `migrate`: This method creates a new table `russenger_user` in the database. It returns a boolean indicating whether the operation was successful.
//! - `create`: This method inserts a new user into the `russenger_user` table. It takes a user ID as an argument and returns a boolean indicating whether the operation was successful.
//! - `set_action`: This method updates the action of a user in the `russenger_user` table. It takes a user ID and an action as arguments and returns a boolean indicating whether the operation was successful.
//!
//! ### Examples
//!
//! ```rust
//! use russenger::models::RussengerUser;
//! use russenger::prelude::*;
//!
//! async fn index(res: Res, req: Req) -> Result<()> {
//!     res.send(TextModel::new(&req.user, "What is your name: ")).await?;
//!     res.redirect("/get_user_input").await?;
//!
//!     Ok(())
//! }
//!
//! async fn get_user_input(res: Res, req: Req) -> Result<()> {
//!     let username: String = req.data.get_value();
//!     res.send(TextModel::new(&req.user, &format!("Hello : {username}"))).await?;
//!     index(res, req).await?; // go back to index Action
//!
//!     Ok(())
//! }
//!
//! #[russenger::main]
//! async fn main() -> Result<()> {
//!     let conn = Database::new().await?.conn;
//!     migrate!([RussengerUser], &conn);
//!     App::init().await?
//!         .attach(
//!             router![
//!                 ("/", index),
//!                 ("/get_user_input", index)
//!              ])
//!         .launch()
//!         .await?;
//!     Ok(())
//! }
//! ```
//!
//! In this example, the `RussengerUser` model is used to represent a user in the Russenger bot framework. The `Query` struct is used to interact with the database. The `Main` action sends a greeting message to the user and sets the next action to `GetUserInput`. The `GetUserInput` action retrieves the user's name and sends a greeting message. The `main` function connects to the database, migrates the database schema, registers the actions for the main application, and launches the main application.
use rusql_alchemy::prelude::*;

/// The `RussengerUser` struct represents a user in the Russenger bot framework.
///
/// This struct has two fields:
///
/// - `facebook_user_id`: A string that represents the user's Facebook user ID. This field is the primary key of the table.
/// - `action`: A string that represents the current action of the user. This field has a default value of "Main".
///
/// The `RussengerUser` struct implements the `FromRow` and `Model` traits from the `rusql_alchemy` crate, which allows it to be used with the `rusql_alchemy` ORM.
///
/// # Examples
///
/// ```rust
/// use rusql_alchemy::prelude::*;
///
/// #[derive(FromRow, Model, Clone, Default)]
/// pub struct RussengerUser {
///     #[model(primary_key = true)]
///     pub facebook_user_id: String,
///
///     #[model(default = "/")]
///     pub action_path: String,
/// }
/// ```
///
/// In this example, the `RussengerUser` struct is defined with two fields: `facebook_user_id` and `action`. The `facebook_user_id` field is marked as the primary key of the table and cannot be null. The `action` field has a default value of "Main". The `RussengerUser` struct implements the `FromRow` and `Model` traits from the `rusql_alchemy` crate, which allows it to be used with the `rusql_alchemy` ORM.
#[derive(FromRow, Model, Clone, Default)]
pub struct RussengerUser {
    #[model(primary_key = true)]
    pub facebook_user_id: String,

    #[model(default = "/")]
    pub action_path: String,
}
