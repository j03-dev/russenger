use crate::query::Query;
use std::collections::HashSet;
use std::sync::Arc;

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

#[derive(Clone)]
pub struct AppState {
    pub query: Query,
    pub action_lock: ActionLock,
}

impl AppState {
    pub async fn init() -> Self {
        let query: Query = Query::new().await;
        let action_lock = ActionLock {
            locked_users: Arc::new(Mutex::new(HashSet::new())),
        };
        Self { query, action_lock }
    }
}
