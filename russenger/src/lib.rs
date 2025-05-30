//! # Russenger Library
//!
//! The Russenger library provides a set of modules and macros to help you build bots using the Russenger bot framework.
//!
//! ## Modules
//!
//! - `cli`: This module provides command-line interface utilities.
//! - `core`: This module contains the core functionalities of the Russenger bot framework.
//! - `prelude`: This module re-exports important traits and structs for convenience.
//! - `query`: This module provides utilities for handling queries.
//! - `response_models`: This module contains models for different types of responses.
//!
//! ## Macros
//!
//! - `action`: This proc macro is used to define an action.
//! - `russenger::actions!`: This macro is used to register the actions for the main application.
//!
//! ## New Features
//!
//! - **Custom Models**: Developers can now use their own models with the Russenger library. This is made possible by the integration with rusql_alchemy, an ORM for sqlx. This means that models are defined in Rust code, eliminating the need to write SQL queries.
//!
//! ## Examples
//!
//! Creating a new action that sends a greeting message when the user input is "Hello":
//!
//! ```rust
//! use russenger::prelude::*;
//!
//! #[derive(FromRow, Clone, Model)]
//! pub struct Register {
//!     #[field(primary_key = true, auto_increment = true)]
//!     pub id: Integer,
//!     #[field(foreign_key = "RussengerUser.facebook_user_id", unique = true)]
//!     pub user_id: String,
//!     #[field(size = 30, unique = true)]
//!     pub username: String,
//! }
//!
//! async fn index(res: Res, req: Req) -> Result<()> {
//!     // Send a greeting message to the user
//!     res.send(TextModel::new(&req.user, "Hello!")).await?;
//!
//!     // Check if the user is registered
//!     if let Some(user_register) = Register::get(kwargs!(user_id = req.user), &req.query.conn).await {
//!         // If the user is registered, send a personalized greeting message
//!         res.send(TextModel::new(&req.user, &format!("Hello {}", user_register.username)))
//!             .await?;
//!     } else {
//!         // If the user is not registered, ask for their name
//!         res.send(TextModel::new(&req.user, "What is your name: "))
//!             .await?;
//!         // Set the next action to SignUp
//!         res.redirect("/signup").await?;
//!         return Ok(());
//!     }
//!
//!     // If the user is registered, set the next action to GetUserInput
//!     res.redirect("/get_user_input").await?;
//!
//!     Ok(())
//! }
//!
//! async fn signup(res: Res, req: Req) -> Result<()> {
//!     // Get the username from the user input
//!     let username: String = req.data.get_value()?;
//!
//!     // Try to create a new Register record for the user
//!     let message = if Register::create(kwargs!(user_id = req.user, username = username), &req.query.conn).await {
//!         "Register success"
//!     } else {
//!         "Register failed"
//!     };
//!
//!     // Send a message to the user indicating whether the registration was successful
//!     res.send(TextModel::new(&req.user, message)).await?;
//!
//!     // Go back to the index action
//!     index(res, req).await?;
//!
//!     Ok(())
//! }
//!
//! async fn get_user_input(res: Res, req: Req) -> Result<()> {
//!     // Define a closure that creates a new Payload for a given value
//!     let payload = |value: &str| Payload::new("/next_action", Some(Data::new(value)));
//!
//!     // Create a QuickReplyModel with two options: "blue" and "red"
//!     let quick_replies: Vec<QuickReply> = vec![
//!         QuickReply::new("blue", None, payload("blue")),
//!         QuickReply::new("red", None, payload("red")),
//!     ];
//!     let quick_reply_model = QuickReplyModel::new(&req.user, "choose one color", quick_replies);
//!
//!     // Send the QuickReplyModel to the user
//!     res.send(quick_reply_model).await?;
//!
//!     Ok(())
//! }
//!
//! async fn next_action(res: Res, req: Req) -> Result<()> {
//!     // Get the color chosen by the user
//!     let color: String = req.data.get_value()?;
//!
//!     // Send a message to the user confirming their choice
//!     res.send(TextModel::new(&req.user, &color)).await?;
//!
//!     // Go back to the index action
//!     index(res, req).await?;
//!
//!     Ok(())
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     App::init().await?
//!        .attach(
//!             Router::new()
//!                 .add("/", index)
//!                 .add("/signup", signup)
//!                 .add("/get_user_input", get_user_input)
//!                 .add("/next_action", next_action)
//!         )
//!         .launch()
//!         .await?;
//!     Ok(())
//! }
//! ```
//!
//! These examples demonstrate how to define an action, use custom models, and register actions for the main application.
pub mod core;
pub mod models;
pub mod prelude;
pub mod query;
pub mod response_models;
pub mod error {
    pub use anyhow::*;
}

pub use core::{
    router::Router,
    services::{webhook_core, webhook_verify}, // core services
};

pub use anyhow;
use anyhow::Context;
pub use rusql_alchemy;

use error::Result;
use query::Query;

use actix_files as fs;
use actix_web::{web, App as ActixApp, HttpServer};
use tokio::sync::Mutex;

use std::{collections::HashSet, sync::Arc};

#[derive(Clone, Default)]
struct ActionLock {
    locked_users: Arc<Mutex<HashSet<String>>>,
}

impl ActionLock {
    async fn lock(&self, user: &str) -> bool {
        let mut locked_user = self.locked_users.lock().await;
        if !locked_user.contains(user) {
            locked_user.insert(user.to_string());
            true
        } else {
            false
        }
    }

    async fn unlock(&self, user: &str) {
        let mut locked_users = self.locked_users.lock().await;
        locked_users.remove(user);
    }
}

/// # App State
///
/// This module contains the `AppState` struct which is used to store the state of the application.
///
/// # Fields
///
/// * `query`: A `Query` that represents the query made by the user.
#[derive(Clone)]
pub struct App {
    query: Arc<Query>,
    router: Arc<Router>,
    action_lock: ActionLock,
    facebook_api_version: String,
    page_access_token: String,
    addr: (String, u16),
}

impl App {
    /// `init` is method to create new `App` instance. in russenger
    pub async fn init() -> Result<Self> {
        let query = Arc::new(Query::new().await?);

        let facebook_api_version = std::env::var("FACEBOOK_API_VERSION").unwrap_or("v19.0".into());
        let page_access_token = std::env::var("PAGE_ACCESS_TOKEN")
            .context("env variable `PAGE_ACCESS_TOKEN` should be set")?;

        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(2453);

        Ok(Self {
            query,
            router: Arc::new(Router::new()),
            action_lock: ActionLock::default(),
            facebook_api_version,
            page_access_token,
            addr: (host, port),
        })
    }

    /// `attach` method allows you to add a group of predefined routes (actions) to the application's router.
    ///
    /// # Parameters
    /// - `router`: A `Router` instance containing a set of routes to be attached.
    ///
    /// #  Examples
    ///
    /// ```rust
    /// use russenger::prelude::*;
    ///
    /// async fn index(res: Res, req: Req) -> Result<()> {
    ///     res.send(TextModel::new(&req.user, "hello world")).await?;
    ///     res.redirect("/next_action").await?;
    ///     Ok(())
    /// }
    ///
    /// async fn next_action(res: Res, req: Req) -> Result<()> {
    ///     let message: String = req.data.get_value()?;
    ///     res.send(TextModel::new(&req.user, &message)).await?; // send the message to the user
    ///     Ok(())
    /// }
    ///
    /// pub fn group_actions() -> Router {
    ///     Router::new()
    ///        .add("/", index)
    ///        .add("/next_action", next_action)
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     App::init().await?
    ///         .attach(group_actions()).await // Attach group actions to the application's router
    ///         .launch().await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// This method is useful for organizing your application's routes into modular and reusable groups.
    pub fn attach(mut self, router: Router) -> Self {
        Arc::get_mut(&mut self.router)
            .expect("Router already shared")
            .routes
            .extend(router.routes);
        self
    }

    pub async fn launch(self) -> Result<()> {
        run_server(self).await?;
        Ok(())
    }
}

fn print_info(host: &str, port: u16) {
    let url = format!("http://{}:{}", host, port);
    println!("Endpoints:");
    println!("  GET: {}/webhook - Webhook verification endpoint", url);
    println!("  POST: {}/webhook - Webhook core endpoint", url);
}

async fn run_server(app: App) -> Result<()> {
    let addr = app.addr.clone();
    print_info(&addr.0, addr.1);
    HttpServer::new(move || {
        ActixApp::new()
            .app_data(web::Data::new(app.clone()))
            .service(webhook_verify)
            .service(webhook_core)
            .service(fs::Files::new("/static", "static").show_files_listing())
    })
    .bind(addr)?
    .run()
    .await?;
    Ok(())
}
