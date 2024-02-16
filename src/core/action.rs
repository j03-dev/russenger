use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::core::request::Req;
use crate::core::response::Res;

#[async_trait::async_trait]
pub trait Action: Send + Sync {
    async fn execute(&self, res: Res, req: Req);

    fn path(&self) -> String;
}

type ActionRegistryType = Arc<Mutex<HashMap<String, Box<dyn Action>>>>;

// Initialize the action registry.
// The action registry is wrapped in an Arc and a Mutex to allow it to be shared and mutated across multiple threads.
lazy_static::lazy_static! {
    pub static ref ACTION_REGISTRY: ActionRegistryType = Arc::new(Mutex::new(HashMap::new()));
}
