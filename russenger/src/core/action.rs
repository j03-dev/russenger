use std::{collections::HashMap, future::Future, pin::Pin};

pub use crate::core::{request::Req, response::Res};
use anyhow::Result;

type FutureResult = Pin<Box<dyn Future<Output = Result<()>>>>;

pub type Action = fn(res: Res, req: Req) -> FutureResult;

pub type Router = HashMap<String, Action>;

pub trait Add {
    fn add(&mut self, path: &str, action: Action);
}

impl Add for Router {
    fn add(&mut self, path: &str, action: Action) {
        self.insert(path.to_owned(), action);
    }
}
