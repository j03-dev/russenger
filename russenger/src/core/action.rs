//! # actions.rs - Action Management for Routing in Russenger
//!
//! The `actions.rs` module is responsible for managing actions and routing them to endpoints
//! in the Russenger framework. It provides the foundational functionality for associating
//! HTTP paths with asynchronous actions, enabling flexible and efficient routing.

use std::{collections::HashMap, future::Future, pin::Pin};

use crate::core::{request::Req, response::Res};
use crate::error::Result;

/// A boxed Future representing the return type of asynchronous action.
///
/// Each `Action` is an asynchronous function that takes a [`Res`] and [`Req`]
/// and resolves to a `Result<()>`. This allows the execution of non-blocking
/// operations within actions.
type FutureResult = Pin<Box<dyn Future<Output = Result<()>>>>;

/// Represents a user-defined "action" in the routing system.
///
/// Actions are asynchronous functions that handle incoming requests and send appropriate
/// responses. Each action receives a [`Res`] (Response object) and [`Req`] (Request object)
/// as arguments.
///
/// Example action:
///
/// ```rust
/// use russenger::prelude::*;
/// use anyhow::Result;
///
/// #[action]
/// async fn greet_user(res: Res, req: Req) -> Result<()> {
///     res.send(TextModel::new(&req.user, "Hello, welcome to Russenger!")).await?;
///     Ok(())
/// }
/// ```
pub type Action = fn(res: Res, req: Req) -> FutureResult;

/// A routing table for associating paths with actions.
///
/// It maps string paths to `Action`, allowing developers to dynamically define the behavior
/// of different endpoints in their application.
///
/// # Example
///
/// ```rust
/// use russenger::prelude::*;
///
/// #[action]
/// async fn greet_user(res: Res, req: Req) -> Result<()> {
///     res.send(TextModel::new(&req.user, "Welcome!")).await?;
///     Ok(())
/// }
///
/// fn group_action() -> Router {
///     Router::new().add("/greet", greet_user)
/// }
///
/// #[russenger::main]
/// async fn main() -> Result<()> {
///     App::init()
///         .await?
///         .attach(router![("/", |res, req| {
///                 Box::pin(async move {
///                     res.send(TextModel::new(&req.user, "Hello World")).await?;
///                     res.redirect("/greet").await?;
///                     Ok(())
///                 })
///             })])
///         .attach(group_action())
///         .launch()
///         .await?;
///     Ok(())
/// }
/// ```
pub type Router = HashMap<String, Action>;

/// A trait for adding routes to the [`Router`].
///
/// This trait provides the `add` method, simplifying the process of associating paths
/// with actions in the router.
///
/// # Example
///
/// ```rust
/// use russenger::prelude::*;
///
///
/// #[action]
/// async fn greet_user(res: Res, req: Req) -> Result<()> {
///     res.send(TextModel::new(&req.user, "Welcome!")).await?;
///     Ok(())
/// }
///
/// async fn group_action() -> Router {
///     router![("greet", greet_user)]
/// }
///
///#[russenger::main]
/// async fn main() -> Result<()> {
///     App::init().await?
///         .attach(router![("/", |res, req| {
///             Box::pin(async move {
///                 res.send(TextModel::new(&req.user, "Hello World")).await?;
///                 res.redirect("/greet").await?;
///                 Ok(())
///             })
///         })])
///         .attach(group_action())
///         .launch()
///         .await?;
///     Ok(())
/// }
/// ```
pub trait Add {
    /// Add a route to the router.
    ///
    /// # Parameters
    /// - `path`: The path to associate with the action, e.g., `/greet`.
    /// - `action`: The handler function (`Action`) to associate with the path.
    ///
    /// # Example
    ///
    /// ```rust
    /// use russenger::prelude::*;
    ///
    /// #[action]
    /// async fn my_action(_res: Res, _req: Req) -> Result<()> {
    ///     Ok(())
    /// }
    /// #[russenger::main]
    /// async fn main() -> Result<()> {
    ///     App::init()
    ///         .await?
    ///         .attach(router![
    ///             ("/path", |_res, _req| { Box::pin(async move { Ok(()) }) }),
    ///             ("/my_action", my_action)
    ///         ])
    ///         .launch()
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    fn add(&mut self, path: &str, action: Action) -> Self;
}

impl Add for Router {
    /// Implementation of the `Add` method for the [`Router`].
    ///
    /// This method inserts the provided path and action into the `HashMap`.
    fn add(&mut self, path: &str, action: Action) -> Self {
        self.insert(path.to_owned(), action);
        self.clone()
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
