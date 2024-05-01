use crate::query::Query;
use crate::response_models::data::Data;

/// The `Req` struct represents a request from a user.
///
/// It contains the following fields:
/// * `user`: A `String` that represents the user who made the request.
/// * `query`: A `Query` that represents the query made by the user.
/// * `data`: A `Data` that represents the data associated with the request.
/// * `host`: A `String` that represents the host from which the request was made.
#[derive(Clone)]
pub struct Req {
    pub user: String,
    pub query: Query,
    pub data: Data,
    pub host: String,
}

impl Req {
    /// Creates a new `Req`.
    ///
    /// # Arguments
    ///
    /// * `user`: A string slice that holds the user.
    /// * `query`: A `Query` that holds the query.
    /// * `data`: A `Data` that holds the data.
    /// * `host`: A string slice that holds the host.
    ///
    /// # Returns
    ///
    /// A `Req` that contains the provided user, query, data, and host.
    pub fn new(user: &str, query: Query, data: Data, host: &str) -> Self {
        Self {
            user: user.to_owned(),
            query,
            data,
            host: host.to_owned(),
        }
    }
}
