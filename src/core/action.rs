use std::collections::HashMap;
use std::sync::Arc;

use rocket::tokio::sync::Mutex;

use crate::core::request::Req;
use crate::core::response::Res;

#[rocket::async_trait]
pub trait Action: Send + Sync {
    async fn execute<'l>(&self, res: Res, req: Req<'l>);
}

type ActionRegistryType = Arc<Mutex<HashMap<&'static str, Box<dyn Action>>>>;

lazy_static::lazy_static! {
    pub static ref ACTION_REGISTRY: ActionRegistryType = Arc::new(Mutex::new(HashMap::new()));
}
