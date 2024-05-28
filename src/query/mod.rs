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
use core::panic;
use std::env::var;

use crate::Action;

use crate::entity::{ActiveModel, Entity as User};

use migration::sea_orm::*;
use migration::sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};

async fn establish_connection() -> Result<DatabaseConnection, String> {
    let database_url = var("DATABASE").expect("Database name not found in .env file");
    Database::connect(&database_url)
        .await
        .map_err(|err| err.to_string())
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
    pub conn: DatabaseConnection,
}

impl Query {
    pub async fn new() -> Self {
        match establish_connection().await {
            Ok(conn) => Self { conn },
            Err(err) => panic!("Can't estabilish the connection {err:?}"),
        }
    }

    pub async fn migrate(&self) -> bool {
        Migrator::up(&self.conn, None).await.is_ok()
    }

    pub async fn create(&self, user_id: &str) -> bool {
        ActiveModel {
            senderid: Set(user_id.to_owned()),
            action: Set("Main".to_owned()),
        }
        .save(&self.conn)
        .await
        .is_ok()
    }
    pub async fn set_action<A: Action>(&self, user_id: &str, action: A) -> bool {
        if let Ok(Some(user)) = User::find_by_id(user_id).one(&self.conn).await {
            ActiveModel {
                senderid: Set(user.senderid),
                action: Set(action.path()),
            }
            .update(&self.conn)
            .await
            .is_ok()
        } else {
            false
        }
    }

    pub async fn get_action(&self, user_id: &str) -> Option<String> {
        if let Ok(Some(user)) = User::find_by_id(user_id).one(&self.conn).await {
            Some(user.action.to_owned())
        } else {
            None
        }
    }
}
