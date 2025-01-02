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
//! async fn index(res: Res, req: Req) -> Result<()> {
//!     res.send(TextModel::new(&req.user, "Hello World")).await?;
//!
//!     Ok(())
//! }
//! ```
use std::env;

use anyhow::Result;
use reqwest::Response;
use serde::Serialize;

use super::request::Req;
use crate::response_models::data::Data;
use crate::response_models::payload::Payload;
use crate::response_models::quick_replies::{QuickReply, QuickReplyModel};
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
/// #[action]
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
    query: Query,
    sender_id: String,
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
        let version = env::var("FACEBOOK_API_VERSION").unwrap_or("v16.0".into());
        let page_access_token =
            env::var("PAGE_ACCESS_TOKEN").expect("env variable `PAGE_ACCESS_TOKEN` should be set");
        let url_api = format!(
            "https://graph.facebook.com/{version}/me/{endpoint}?access_token={page_access_token}",
            endpoint = response_model.get_endpoint()
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

    pub fn new(sender_id: &str, query: Query) -> Self {
        Self {
            query,
            sender_id: sender_id.to_owned(),
        }
    }

    pub async fn redirect(&self, path: &str) -> Result<()> {
        self.query.set_path(&self.sender_id, path).await;
        Ok(())
    }
}

pub async fn send_next(path: &str, res: Res, req: Req) -> Result<()> {
    let mut page = req.data.get_page().unwrap_or_default();
    page.next();

    let quick_reply = QuickReplyModel::new(
        &req.user,
        "Navigation",
        vec![QuickReply::new(
            "Next",
            None,
            Payload::new(
                path,
                Some(Data::new(req.data.get_value::<String>(), Some(page))),
            ),
        )],
    );

    res.send(quick_reply).await?;

    Ok(())
}

async fn fetch_post<T: Serialize>(url: &str, body: T) -> Result<Response> {
    Ok(reqwest::Client::new().post(url).json(&body).send().await?)
}
