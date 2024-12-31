use std::{
    collections::{HashMap, HashSet},
    future::Future,
    pin::Pin,
    sync::Arc,
};

pub use crate::core::{request::Req, response::Res};
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

pub type Action = fn(res: Res, req: Req) -> Pin<Box<dyn Future<Output = Result<()>>>>;

pub type Router = HashMap<String, Action>;

lazy_static::lazy_static! {
    pub static ref app: Arc<Mutex<Router>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref action_lock: ActionLock = ActionLock { locked_users: Arc::new(Mutex::new(HashSet::default())) };
}

pub trait Add {
    fn add(&mut self, path: &str, handler: Action);
}

impl Add for Router {
    fn add(&mut self, path: &str, handler: Action) {
        self.insert(path.to_string(), handler);
    }
}
