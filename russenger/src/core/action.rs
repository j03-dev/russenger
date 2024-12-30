use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

pub use crate::core::{request::Req, response::Res};
use anyhow::Result;
use tokio::sync::Mutex;

pub type Action = fn(res: Res, req: Req) -> Pin<Box<dyn Future<Output = Result<()>>>>;

pub type Router = HashMap<String, Action>;

lazy_static::lazy_static! {
    pub static ref app: Arc<Mutex<Router>> = Arc::new(Mutex::new(HashMap::new()));
}

pub trait Add {
    fn add(&mut self, path: &str, handler: Action);
}

impl Add for Router {
    fn add(&mut self, path: &str, handler: Action) {
        self.insert(path.to_string(), handler);
    }
}
