//! The `app_state` module provides the `AppState` struct.
//!
//! The `AppState` struct is utilized by Actix's `AppState` to maintain the application's state.
//! It contains the following fields:
//! - `query`: A `Query` instance used to interact with the database.
use std::{collections::HashSet, sync::Arc};

use crate::query::Query;
use anyhow::Result;
use tokio::sync::Mutex;

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
pub struct AppState {
    pub templates: tera::Tera,
    pub query: Query,
    pub action_lock: ActionLock,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        let query: Query = Query::new().await?;
        let template_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*");
        let templates = tera::Tera::new(template_dir)?;
        let action_lock = ActionLock {
            locked_users: Arc::new(Mutex::new(HashSet::new())),
        };
        Ok(Self {
            query,
            templates,
            action_lock,
        })
    }
}
