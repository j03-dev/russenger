use std::collections::HashMap;
use std::sync::Arc;

use rocket::tokio::sync::Mutex;

use crate::core::request::Req;
use crate::core::response::Res;

// The Action trait represents an action that can be executed.
// It requires the implementer to be Send and Sync, meaning it can be sent between threads and accessed from multiple threads.
#[rocket::async_trait]
pub trait Action: Send + Sync {
    // The execute method takes a response and a request, and performs the action.
    // This method is asynchronous, meaning it can perform I/O operations or other long-running tasks without blocking the current thread.
    async fn execute(&self, res: Res, req: Req);

    // The path method returns the path of the action.
    // This is used to identify the action in the action registry.
    fn path(&self) -> String;
}

// Define the type for the action registry.
// The action registry is a thread-safe hashmap that maps action paths to actions.
type ActionRegistryType = Arc<Mutex<HashMap<String, Box<dyn Action>>>>;

// Initialize the action registry.
// The action registry is wrapped in an Arc and a Mutex to allow it to be shared and mutated across multiple threads.
lazy_static::lazy_static! {
    pub static ref ACTION_REGISTRY: ActionRegistryType = Arc::new(Mutex::new(HashMap::new()));
}
