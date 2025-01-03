//! The `app_state` module provides the `AppState` struct.
//!
//! The `AppState` struct is utilized by Actix's `AppState` to maintain the application's state.
//! It contains the following fields:
//! - `query`: A `Query` instance used to interact with the database.
use std::{collections::HashSet, sync::Arc};

use crate::query::Query;
use anyhow::Result;
use tokio::sync::Mutex;

use super::action::{Action, Router};

#[derive(Clone)]
pub struct ActionLock {
    pub locked_users: Arc<Mutex<HashSet<String>>>,
}

impl ActionLock {
    pub async fn lock(&self, user: &str) -> bool {
        let mut locked_user = self.locked_users.lock().await;
        if !locked_user.contains(user) {
            locked_user.insert(user.to_string());
            true
        } else {
            false
        }
    }

    pub async fn unlock(&self, user: &str) {
        let mut locked_users = self.locked_users.lock().await;
        locked_users.remove(user);
    }
}

/// # App State
///
/// This module contains the `AppState` struct which is used to store the state of the application.
///
/// # Fields
///
/// * `query`: A `Query` that represents the query made by the user.
#[derive(Clone)]
pub struct App {
    pub(crate) query: Query,
    pub(crate) router: Arc<Mutex<Router>>,
    pub(crate) action_lock: ActionLock,
}

impl App {
    /// `init` is method to create new `App` instance. in russenger
    pub async fn init() -> Result<Self> {
        let query: Query = Query::new().await?;
        Ok(Self {
            query,
            router: Arc::new(Mutex::new(Router::new())),
            action_lock: ActionLock {
                locked_users: Arc::new(Mutex::new(HashSet::new())),
            },
        })
    }

    /// `add` is method to add action(handler) to router
    ///
    /// # Parameters
    /// * `path: &str` - The key of the action
    /// * `action: Action` - The action handler
    ///
    /// Examples
    ///
    /// add new action on app
    ///
    /// ```rust
    /// use russenger::prelude::*;
    /// #[action]
    /// async fn get_user_input(res: Res, req: Req) -> Result<()> {
    ///     let name: String = req.data.get_value();
    ///     let message = format!("Hello {name}");
    ///     res.send(TextModel::new(&req.user, &message)).await?;
    ///     Ok(())
    /// }
    ///
    /// #[russenger::main]
    /// async fn main() -> Result<()> {
    ///     let mut app = App::init().await?;
    ///     app.add("/", |res: Res, req: Req| {
    ///         Box::pin(async move {
    ///             res.send(GetStartedButtonModel::new(Payload::default())).await?;
    ///             res.send(TextModel::new(&req.user, "Hello World")).await?;
    ///             res.redirect("/get_user_input").await?;
    ///             Ok(())
    ///         })
    ///     }).await;
    ///     app.add("/get_user_input", get_user_input).await;
    ///     Ok(())
    /// }
    /// ````
    pub async fn add(&mut self, path: &str, action: Action) {
        self.router.lock().await.insert(path.to_owned(), action);
    }
}
