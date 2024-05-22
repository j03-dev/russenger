//! The `app_state` module contains the `AppState`
//!
//! The `AppState` struct is use to store the state of the application.
//! query:  `Query` is use to query the database
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
    pub query: Query,
}

impl AppState {
    pub async fn init() -> Self {
        let query: Query = Query::new().await;
        Self { query }
    }
}
