use std::{collections::HashMap, future::Future, pin::Pin};

pub use crate::core::{request::Req, response::Res};
use anyhow::Result;

pub type Action = fn(res: Res, req: Req) -> Pin<Box<dyn Future<Output = Result<()>>>>;

pub type Router = HashMap<String, Action>;

pub trait Add {
    fn add(&mut self, path: &str, handler: Action);
}

impl Add for Router {
    fn add(&mut self, path: &str, handler: Action) {
        self.insert(path.to_string(), handler);
    }
}
