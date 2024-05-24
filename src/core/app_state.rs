//! The `app_state` module provides the `AppState` struct.
//!
//! The `AppState` struct is utilized by Actix's `AppState` to maintain the application's state.
//! It contains the following fields:
//! - `query`: A `Query` instance used to interact with the database.
use crate::query::Query;

/// # App State
///
/// This module contains the `AppState` struct which is used to store the state of the application.
///
/// # Fields
///
/// * `query`: A `Query` that represents the query made by the user.
#[derive(Clone)]
pub struct AppState {
    pub templates: tera::Tera,
    pub query: Query,
}

impl AppState {
    pub async fn init() -> Self {
        let query: Query = Query::new().await;
        let template_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*");
        let templates = tera::Tera::new(template_dir).unwrap();
        Self { query, templates }
    }
}
