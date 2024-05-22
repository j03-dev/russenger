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

use sqlx::{MySql, Pool, Postgres, Row, Sqlite};

use crate::Action;

#[derive(Clone)]
pub enum DB {
    Mysql(Pool<MySql>),
    Postgres(Pool<Postgres>),
    Sqlite(Pool<Sqlite>),
    Null,
}

type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

async fn establish_connection() -> DbResult<DB> {
    let database_url = var("DATABASE").expect("Database name not found in .env file");
    let msg = "Database connection failed";

    let engine = database_url.split(':').next().ok_or(msg)?;

    let pool = match engine {
        "mysql" => Pool::connect(&database_url).await.map(DB::Mysql)?,
        "postgres" | "postgresql" => Pool::connect(&database_url).await.map(DB::Postgres)?,
        "sqlite" => Pool::connect(&database_url).await.map(DB::Sqlite)?,
        _ => DB::Null,
    };

    Ok(pool)
}

macro_rules! execute_query {
    ($pool:expr, $sql:expr, $params:expr) => {{
        let mut query = sqlx::query($sql);
        for parm in $params {
            query = query.bind(parm);
        }
        query.execute($pool).await.is_ok()
    }};
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
    pub db: DB,
}

impl Query {
    /// Creates a new `Query`.
    ///
    /// This method establishes a connection to the database and returns a `Query` with the established connection.
    ///
    /// # Returns
    ///
    /// * `Query`: The created `Query`.
    pub async fn new() -> Self {
        match establish_connection().await {
            Ok(db) => Self { db },
            Err(err) => panic!("Can't estabilish the connection {err:?}"),
        }
    }

    /// Creates a new table `russenger_user` in the database.
    ///
    /// This method returns a boolean indicating whether the operation was successful.
    ///
    /// # Returns
    ///
    /// * `bool`: Whether the operation was successful.
    pub async fn migrate(&self) -> bool {
        let sql = "
            create table russenger_user (
                facebook_user_id varchar(40) primary key unique,
                action varchar(20)
            );";

        let no_params: [&str; 0] = [];
        match &self.db {
            DB::Mysql(pool) => execute_query!(pool, sql, no_params),
            DB::Sqlite(pool) => execute_query!(pool, sql, no_params),
            DB::Postgres(pool) => execute_query!(pool, sql, no_params),
            DB::Null => false,
        }
    }

    /// Inserts a new user into the `russenger_user` table.
    ///
    /// This method takes a user ID as an argument and returns a boolean indicating whether the operation was successful.
    ///
    /// # Arguments
    ///
    /// * `user_id`: The user ID of the new user.
    ///
    /// # Returns
    ///
    /// * `bool`: Whether the operation was successful.
    pub async fn create(&self, user_id: &str) -> bool {
        let params = [user_id, "Main"];
        match &self.db {
            DB::Mysql(pool) => {
                let sql = "insert into russenger_user (facebook_user_id, action) values (?, ?)";
                execute_query!(pool, sql, params)
            }
            DB::Sqlite(pool) => {
                let sql = "insert into russenger_user (facebook_user_id, action) values ($1, $2)";
                execute_query!(pool, sql, params)
            }
            DB::Postgres(pool) => {
                let sql = "insert into russenger_user (facebook_user_id, action) values ($1, $2)";
                execute_query!(pool, sql, [user_id, "Main"])
            }
            DB::Null => false,
        }
    }
    /// Updates the action of a user in the `russenger_user` table.
    ///
    /// This method takes a user ID and an action as arguments and returns a boolean indicating whether the operation was successful.
    ///
    /// # Arguments
    ///
    /// * `user_id`: The user ID of the user whose action is to be updated.
    /// * `action`: The new action for the user.
    ///
    /// # Returns
    ///
    /// * `bool`: Whether the operation was successful.
    ///
    /// # Examples
    ///
    /// Updating the action of a user:
    ///
    /// ```rust
    /// use russenger::prelude::*;
    ///
    /// #[action]
    /// async fn Main(res: Res, req: Req) {
    ///    req.query.set_action(&req.user, NextAction).await;
    /// }
    ///
    /// #[action]
    /// async fn NextAction(res: Res, req: Req) {}
    ///
    /// russenger_app!(Main, NextAction);
    /// ```
    ///
    /// # Database Queries
    ///
    /// This method executes different SQL queries based on the type of the database:
    ///
    /// * For MySQL: `"update russenger_user set action=? where facebook_user_id=?"`
    /// * For SQLite: `"update russenger_user set action=$1 where facebook_user_id=$2"`
    /// * For Postgres: `"update russenger_user set action=$1 where facebook_user_id=$2"`
    pub async fn set_action<A: Action>(&self, user_id: &str, action: A) -> bool {
        let params = [action.path(), user_id.to_string()];
        match &self.db {
            DB::Mysql(pool) => {
                let sql = "update russenger_user set action=? where facebook_user_id=?";
                execute_query!(pool, sql, params)
            }
            DB::Sqlite(pool) => {
                let sql = "update russenger_user set action=$1 where facebook_user_id=$2";
                execute_query!(pool, sql, params)
            }
            DB::Postgres(pool) => {
                let sql = "update russenger_user set action=$1 where facebook_user_id=$2";
                execute_query!(pool, sql, params)
            }
            DB::Null => false,
        }
    }

    /// Retrieves the action of a user from the `russenger_user` table.
    ///
    /// This method takes a user ID as an argument and returns the action of the user if it exists.
    ///
    /// # Arguments
    ///
    /// * `user_id`: The user ID of the user whose action is to be retrieved.
    ///
    /// # Returns
    ///
    /// * `Option<String>`: The action of the user if it exists, or `None` if it doesn't.
    ///
    /// # Database Queries
    ///
    /// This method executes different SQL queries based on the type of the database:
    ///
    /// * For MySQL: `"select action from russenger_user where facebook_user_id=?"`
    /// * For SQLite: `"select action from russenger_user where facebook_user_id=$1"`
    /// * For Postgres: `"select action from russenger_user where facebook_user_id=$1"`
    pub async fn get_action(&self, user_id: &str) -> Option<String> {
        match &self.db {
            DB::Mysql(pool) => {
                let sql = "select action from russenger_user where facebook_user_id=?";
                match sqlx::query(sql).bind(user_id).fetch_one(pool).await {
                    Ok(row) => row.get(0),
                    Err(_) => None,
                }
            }
            DB::Sqlite(pool) => {
                let sql = "select action from russenger_user where facebook_user_id=$1";
                match sqlx::query(sql).bind(user_id).fetch_one(pool).await {
                    Ok(row) => row.get(0),
                    Err(_) => None,
                }
            }
            DB::Postgres(pool) => {
                let sql = "select action from russenger_user where facebook_user_id=$1";
                match sqlx::query(sql).bind(user_id).fetch_one(pool).await {
                    Ok(row) => row.get(0),
                    Err(_) => None,
                }
            }
            DB::Null => None,
        }
    }
}
