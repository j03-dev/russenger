//! The `app_state` module provides the `AppState` struct.
//!
//! The `AppState` struct is utilized by Actix's `AppState` to maintain the application's state.
//! It contains the following fields:
//! - `query`: A `Query` instance used to interact with the database.
use std::{collections::HashSet, sync::Arc};

use crate::query::Query;
use anyhow::Result;
use tokio::sync::Mutex;

use super::action::Router;

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
    pub(crate) tmp_router: Router,
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
            tmp_router: Router::new(),
        })
    }

    /// `attach` method allows you to add a group of predefined routes (actions) to the application's router.
    ///
    /// # Parameters
    /// - `router`: A `Router` instance containing a set of routes to be attached.
    ///
    /// #  Examples
    ///
    /// ```rust
    /// use russenger::prelude::*;
    ///
    /// #[action]
    /// async fn index(res: Res, req: Req) -> Result<()> {
    ///     res.send(TextModel::new(&req.user, "hello world")).await?;
    ///     res.redirect("/next_action").await?;
    ///     Ok(())
    /// }
    ///
    /// #[action]
    /// async fn next_action(res: Res, req: Req) -> Result<()> {
    ///     let message: String = req.data.get_value();
    ///     res.send(TextModel::new(&req.user, &message)).await?; // send the message to the user
    ///     Ok(())
    /// }
    ///
    /// pub fn group_actions() -> Router {
    ///     let mut router = Router::new();
    ///     router.add("/", index);
    ///     router.add("/next_action", next_action);
    ///     router
    /// }
    ///
    /// #[russenger::main]
    /// async fn main() -> Result<()> {
    ///     let mut app = App::init().await?;
    ///     app.attach(group_actions()).await; // Attach group actions to the application's router
    ///     launch(app).await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// This method is useful for organizing your application's routes into modular and reusable groups.
    pub fn attach(&mut self, router: Router) -> &mut Self {
        self.tmp_router.extend(router);
        self
    }
}
