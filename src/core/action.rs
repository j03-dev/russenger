use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{Req, Res};

#[async_trait::async_trait]
pub trait Action: Send + Sync {
    async fn execute(&self, res: Res, req: Req);

    fn path(&self) -> String;
}

type ActionRegistryType = Arc<Mutex<HashMap<String, Box<dyn Action>>>>;

lazy_static::lazy_static! {
    pub static ref ACTION_REGISTRY: ActionRegistryType = Arc::new(Mutex::new(HashMap::new()));
    pub static ref ACTION_LOCK: ActionLock = ActionLock { locked_users: Arc::new(Mutex::new(HashSet::new()))};
}
