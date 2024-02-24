use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use rocket::tokio::sync::Mutex;

use crate::{Req, Res};

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

#[rocket::async_trait]
pub trait Action: Send + Sync {
    async fn execute(&self, res: Res, req: Req);

    fn path(&self) -> String;
}

type ActionRegistryType = Arc<Mutex<HashMap<String, Box<dyn Action>>>>;

lazy_static::lazy_static! {
    pub static ref ACTION_REGISTRY: ActionRegistryType = Arc::new(Mutex::new(HashMap::new()));
    pub static ref ACTION_LOCK: ActionLock = ActionLock { locked_users: Arc::new(Mutex::new(HashSet::new()))};
}
