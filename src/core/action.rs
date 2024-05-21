use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tokio::sync::Mutex;

use super::{request::Req, response::Res};
use crate::response_models::data::Data;
use crate::response_models::payload::Payload;
use crate::response_models::quick_replies::{QuickReply, QuickReplyModel};

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

/// The `IsExist` trait defines the behavior of checking if an action exists.
///
/// # Methods
///
/// * `is_exist`: This method is called to check if an action exists.
///
/// # Examples
///
/// ```rust
/// use russenger::prelude::*;
///
/// #[action]
/// async fn HelloWorld(res: Res, req: Req) {
///    res.send(TextModel::new(&req.user, "Hello, world!")).await;
/// }
///
/// #[russenger::main]
/// async fn main() {
///     let path = Path::from("HelloWorld");
///     let is_exist = path.is_exist().await;
///     assert_eq!(is_exist, true);
/// }
/// ```
#[async_trait::async_trait]
pub trait IsExist {
    async fn is_exist(&self) -> bool;
}

/// The `Path` type represents the path of an action.
///
/// The path is a unique identifier for an action. It is used to route requests to the appropriate action.
///
/// # Examples
///
/// ```rust
/// use russenger::prelude::*;
///
/// let path = Path::from("HelloWorld");
/// ```
pub type Path = String;

#[async_trait::async_trait]
impl IsExist for Path {
    /// Checks if an action exists.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the action exists.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use russenger::prelude::*;
    ///
    /// #[action]
    /// async fn HelloWorld(res: Res, req: Req) {
    ///   res.send(TextModel::new(&req.user, "Hello, world!")).await;
    /// }
    ///
    /// #[russenger::main]
    /// async fn main() {
    ///     let path = Path::from("HelloWorld");
    ///     let is_exist = path.is_exist().await;
    ///     assert_ne!(is_exist, true);
    /// }
    /// ```
    async fn is_exist(&self) -> bool {
        ACTION_REGISTRY.lock().await.contains_key(self)
    }
}

/// The `Action` trait defines the behavior of an action.
///
/// An action is a unit of work that the application can perform. Each action is associated with a path, and when a request is received with that path, the action's `execute` method is called.
///
/// # Methods
///
/// * `execute`: This method is called when a request is received with the action's path. It takes a `Res` and a `Req` as arguments, which represent the response and request respectively.
/// * `path`: This method returns the path associated with the action.
///
/// # Examples
///
/// ```rust
/// use russenger::prelude::*;
///
/// #[action]
/// async fn Greet(res: Res, req: Req) {
///     let message: String = req.data.get_value();
///
///     if message == "Hello" {
///         res.send(TextModel::new(&req.user, "Hello, welcome to our bot!")).await;
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait Action: Send + Sync {
    async fn execute(&self, res: Res, req: Req);

    fn path(&self) -> Path;

    async fn next(&self, res: Res, req: Req) {
        let mut page = req.data.get_page().unwrap_or_default();
        page.next(); // increment the page number
        let data = Data::new(req.data.get_value::<String>(), Some(page));
        let payload = Payload::from_path(self.path(), Some(data)).await;
        let quick_reply: QuickReplyModel<'_> = QuickReplyModel::new(
            &req.user,
            "Navigation",
            vec![QuickReply::new("Next", "", payload)],
        );
        res.send(quick_reply).await;
    }
}

type ActionRegistryType = Arc<Mutex<HashMap<Path, Box<dyn Action>>>>;

lazy_static::lazy_static! {
    /// `ACTION_REGISTRY` is a thread-safe map that stores all the actions available in the application.
    ///
    /// Each action is associated with a path, and when a request is received with that path, the corresponding action's `execute` method is called.
    ///
    /// The `ACTION_REGISTRY` is initialized with an empty map when the application starts, and actions are added to it using the `create_action!` macro.
    ///
    /// # Examples
    ///
    /// Adding an action to the `ACTION_REGISTRY`:
    /// ```rust
    /// use russenger::{ACTION_REGISTRY, Action};
    ///
    /// use russenger::prelude::*;
    ///
    /// #[action]
    /// async fn Main(res: Res, req: Req) {
    ///     let message: String = req.data.get_value();
    ///
    ///     if message == "Hello" {
    ///         res.send(TextModel::new(&req.user, "Hello, welcome to our bot!")).await;
    ///     }
    /// }
    ///
    /// #[russenger::main]
    /// async fn main() {
    ///     ACTION_REGISTRY.lock().await.insert(Main.path(), Box::new(Main));
    /// }
    /// ```
    pub static ref ACTION_REGISTRY: ActionRegistryType = Arc::new(Mutex::new(HashMap::new()));
    pub static ref ACTION_LOCK: ActionLock = ActionLock { locked_users: Arc::new(Mutex::new(HashSet::new()))};
}
