//! The `query` module provides utilities for handling queries.
//! The `Query` struct represents a database query. It includes a `DB` enum that represents the database connection.
//!
//! ## new Method
//!
//! The `new` method establishes a connection to the database and returns a new `Query` with the established connection.
//!
//! ## migrate Method
//!
//! The `migrate` method creates a new table `russenger_user` in the database. It returns a boolean indicating whether the operation was successful.
//!
//! ## create Method
//!
//! The `create` method inserts a new user into the `russenger_user` table. It takes a user ID as an argument and returns a boolean indicating whether the operation was successful.
//!
//! ## set_action Method
//!
//! The `set_action` method updates the action of a user in the `russenger_user` table. It takes a user ID and an action as arguments and returns a boolean indicating whether the operation was successful.
//!
//! ## Examples
//!
//! ```rust
//! use russenger::models::RussengerUser;
//! use russenger::prelude::*;
//!
//! #[action]
//! async fn Main(res: Res, req: Req) {
//!     res.send(TextModel::new(&req.user, "What is your name: ")).await?;
//!     req.query.set_action(&req.user, GetUserInput).await;
//!
//!     Ok(())
//! }
//!
//! #[action]
//! async fn GetUserInput(res: Res, req: Req) {
//!     let username: String = req.data.get_value();
//!     res.send(TextModel::new(&req.user, &format!("Hello : {username}"))).await;
//!     Main.execute(res, req).await?; // go back to Main Action
//!
//!     Ok(())
//! }
//!
//! #[russenger::main]
//! async fn main() {
//!     let conn = Database::new().await.conn;
//!     migrate!([RussengerUser], &conn);
//!     russenger::actions![Main, GetUserInput];
//!     russenger::launch().await.ok();
//! }
//! ```
use crate::models::RussengerUser;
use anyhow::Result;

use rusql_alchemy::prelude::*;

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
#[derive(Clone)]
pub struct Query {
    pub conn: Connection,
}

/// Represents a query object used for database operations.
impl Query {
    /// Creates a new Query object.
    ///
    /// # Returns
    ///
    /// Returns a Query object if the connection is successfully established.
    ///
    /// # Panics
    ///
    /// Panics if the connection cannot be established.
    pub(crate) async fn new() -> Result<Self> {
        let conn = Database::new().await?.conn;
        Ok(Self { conn })
    }

    /// Runs database migrations.
    ///
    /// # Returns
    ///
    /// Returns `true` if the migrations are successful, `false` otherwise.
    pub async fn migrate(&self) -> bool {
        migrate!([RussengerUser], &self.conn)
    }

    /// Creates a new record in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user.
    ///
    /// # Returns
    ///
    /// Returns `true` if the record is successfully created, `false` otherwise.
    pub async fn create(&self, user_id: &str) -> bool {
        if (RussengerUser::get(kwargs!(facebook_user == user_id), &self.conn).await).is_none() {
            return RussengerUser::create(kwargs!(facebook_user_id = user_id), &self.conn).await;
        }
        true
    }

    /// Sets the action for a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user.
    /// * `action` - The action to set.
    ///
    /// # Returns
    ///
    /// Returns `true` if the action is successfully set, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use russenger::models::RussengerUser;
    /// use russenger::prelude::*;
    ///
    /// #[action]
    /// async fn Main(res: Res, req: Req) {
    ///     res.send(TextModel::new(&req.user, "What is your name: ")).await?;
    ///     req.query.set_action(&req.user, GetUserInput).await;
    ///
    ///     Ok(())
    /// }
    ///
    /// #[action]
    /// async fn GetUserInput(res: Res, req: Req) {
    ///     let username: String = req.data.get_value();
    ///     res.send(TextModel::new(&req.user, &format!("Hello : {username}"))).await?;
    ///
    ///     Ok(())
    /// }
    ///
    /// #[russenger::main]
    /// async fn main() {
    ///     let conn = Database::new().await.conn;
    ///     migrate!([RussengerUser], &conn);
    ///     russenger::actions![Main, GetUserInput];
    ///     russenger::launch().await.ok();
    /// }
    /// ```
    pub(crate) async fn set_path(&self, user_id: &str, path: &str) -> bool {
        if let Some(mut user) =
            RussengerUser::get(kwargs!(facebook_user_id == user_id), &self.conn).await
        {
            user.action_path = path.to_owned();
            user.update(&self.conn).await
        } else {
            false
        }
    }

    /// Retrieves the action for a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user.
    ///
    /// # Returns
    ///
    /// Returns the action as an `Option<String>`. Returns `None` if the user is not found.
    pub async fn get_path(&self, user_id: &str) -> Option<String> {
        RussengerUser::get(kwargs!(facebook_user_id == user_id), &self.conn)
            .await
            .map(|user| user.action_path)
    }
}
