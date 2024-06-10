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
//! use russenger::prelude::*;
//!
//! #[action]
//! async fn Main(res: Res, req: Req) {
//!     res.send(TextModel::new(&req.user, "What is your name: ")).await;
//!     req.query.set_action(&req.user, GetUserInput).await;
//! }
//!
//! #[action]
//! async fn GetUserInput(res: Res, req: Req) {
//!     let username: String = req.data.get_value();
//!     res.send(TextModel::new(&req.user, &format!("Hello : {username}"))).await;
//!     Main.execute(res, req).await; // go back to Main Action
//! }
//!
//! russenger_app!(Main, GetUserInput);
//! ```
use rusql_alchemy::prelude::*;

use crate::Action;

#[derive(Deserialize, Model, Default)]
pub struct RussengerUser {
    #[model(primary_key = true, null = false)]
    pub facebook_user_id: String,
    #[model(default = "Main")]
    pub action: String,
}

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
    pub async fn new() -> Self {
        let conn = config::db::Database::new().await.conn;
        Self { conn }
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
        RussengerUser::create(kwargs!(facebook_user_id = user_id), &self.conn).await
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
    /// use russenger::prelude::*;
    ///
    /// #[action]
    /// async fn Main(res: Res, req: Req) {
    ///     res.send(TextModel::new(&req.user, "What is your name: ")).await;
    ///     req.query.set_action(&req.user, GetUserInput).await;
    /// }
    ///
    /// #[action]
    /// async fn GetUserInput(res: Res, req: Req) {
    ///     let username: String = req.data.get_value();
    ///     res.send(TextModel::new(&req.user, &format!("Hello : {username}"))).await;
    /// }
    ///
    /// russenger_app!(Main, GetUserInput);
    /// ```
    pub async fn set_action<A: Action>(&self, user_id: &str, action: A) -> bool {
        if let Some(user) =
            RussengerUser::get(kwargs!(facebook_user_id = user_id), &self.conn).await
        {
            RussengerUser {
                action: action.path(),
                ..user
            }
            .update(&self.conn)
            .await
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
    pub async fn get_action(&self, user_id: &str) -> Option<String> {
        if let Some(user) =
            RussengerUser::get(kwargs!(facebook_user_id = user_id), &self.conn).await
        {
            Some(user.action)
        } else {
            None
        }
    }
}
