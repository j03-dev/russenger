use std::{collections::HashMap, future::Future, pin::Pin};

use crate::core::{request::Req, response::Res};
use crate::error::Result;

type FutureResult = Pin<Box<dyn Future<Output = Result<()>> + Send>>;

pub struct Router {
    pub routes: HashMap<String, Box<dyn Fn(Res, Req) -> FutureResult + Send + Sync>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add<F, Fut>(mut self, path: &str, action: F) -> Self
    where
        F: Fn(Res, Req) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let boxed =
            Box::new(move |res: Res, req: Req| -> FutureResult { Box::pin(action(res, req)) });

        self.routes.insert(path.to_owned(), boxed);
        self
    }
}

#[macro_export]
macro_rules! router {
    ( $( ($path:expr, $action:expr) ),* $(,)? ) => {
        {
            russenger::prelude::Router::new()
            $(
                .add($path, $action)
            )*
        }
    };
}
