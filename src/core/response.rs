//! The `response` module contains the `Res` struct that represents a response that can be sent to a user.
//!
//! The `Res` struct does not contain any fields.
//!
//! # Examples
//!
//! Send a response to a user on Action
//!
//! ```rust
//! use russenger::prelude::*;
//!
//! #[action]
//! async fn Main(res: Res, req: Req) {
//!     res.send(TextModel::new(&req.user, "Hello World")).await?;
//!
//!     Ok(())
//! }
//! ```
use std::env;

use anyhow::{bail, Result};

use crate::response_models::ResponseModel;

/// The `Res` struct represents a response that can be sent to a user.
///
/// It does not contain any fields.
///
/// # Examples
///
/// Sending a response to a user:
///
/// ```rust
/// use russenger::response_models::text::TextModel;
/// use russenger::core::response::Res;
///
/// let res = Res;
/// let response_model = TextModel::new("sender_id", "Hello, user1!");
///
/// let send_result = res.send(response_model);
/// ```
///
/// # Methods
///
/// * `send`: Sends a response to a user. It takes a `ResponseModel` as an argument and returns a `SendResult`.
pub struct Res;

impl Res {
    /// Sends a response to a user.
    ///
    /// # Arguments
    ///
    /// * `response_model`: A `ResponseModel` that represents the response to be sent.
    ///
    /// # Returns
    ///
    /// A `SendResult` that represents the result of the send operation.
    ///
    /// # Errors
    ///
    /// Returns `SendResult::Error` if the send operation fails.
    pub async fn send<T: ResponseModel>(&self, response_model: T) -> Result<String> {
        let version = env::var("FACEBOOK_API_VERSION").unwrap_or("v15.0".into());
        let page_access_token =
            env::var("PAGE_ACCESS_TOKEN").expect("env variable `PAGE_ACCESS_TOKEN` should be set");
        let url_api = format!(
            "https://graph.facebook.com/{version}/me/{endpoint}?access_token={page_access_token}",
            endpoint = response_model.get_endpoint()
        );
        match reqwest::Client::new()
            .post(url_api)
            .json(&response_model)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_client_error() {
                    bail!(response.text().await.unwrap())
                } else {
                    Ok(response.text().await.unwrap())
                }
            }
            Err(err) => bail!(err.to_string()),
        }
    }
}
