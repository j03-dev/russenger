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
//! async fn index(res: Res, req: Req) -> Result<()> {
//!     res.send(TextModel::new(&req.user, "Hello World")).await?;
//!
//!     Ok(())
//! }
//! ```
use std::sync::Arc;

use anyhow::Result;
use reqwest::Response;
use serde::Serialize;

use crate::{query::Query, response_models::ResponseModel};

/// The `Res` struct represents a response that can be sent to a user.
///
/// It does not contain any fields.
///
/// # Examples
///
/// Sending a response to a user:
///
/// ```rust
/// use russenger::prelude::*;
///
/// async fn index(res: Res, req: Req) -> Result<()> {
///     let response_model = TextModel::new("sender_id", "Hello, user1!");
///     let send_result = res.send(response_model).await?;
///     Ok(())
/// }
///
/// ```
///
/// # Methods
///
/// * `send`: Sends a response to a user. It takes a `ResponseModel` as an argument and returns a `SendResult`.
pub struct Res {
    query: Arc<Query>,
    sender_id: String,
    facebook_api_version: String,
    page_access_token: String,
}

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
        let url_api = format!(
            "https://graph.facebook.com/{version}/me/{endpoint}?access_token={token}",
            endpoint = response_model.get_endpoint(),
            version = self.facebook_api_version,
            token = self.page_access_token
        );

        match fetch_post(&url_api, response_model).await {
            Ok(response) => {
                if response.status().is_client_error() {
                    Err(anyhow::anyhow!(response.text().await?))
                } else {
                    Ok(response.text().await?)
                }
            }
            Err(err) => Err(anyhow::anyhow!(err)),
        }
    }

    pub fn new(
        sender_id: &str,
        query: Arc<Query>,
        facebook_api_version: String,
        page_access_token: String,
    ) -> Self {
        Self {
            query,
            sender_id: sender_id.to_owned(),
            facebook_api_version,
            page_access_token,
        }
    }

    pub async fn redirect(&self, path: &str) -> Result<()> {
        self.query.set_path(&self.sender_id, path).await?;
        Ok(())
    }
}

async fn fetch_post<T: Serialize>(url: &str, body: T) -> Result<Response> {
    Ok(reqwest::Client::new().post(url).json(&body).send().await?)
}
