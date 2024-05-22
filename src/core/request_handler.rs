//! The `request_handler` module contains the request handler for the webhook endpoint.
//!
//! The `WebQuery` struct is used to handle the query parameters of the webhook endpoint.
//! It specifically deals with the parameters sent by Facebook during the webhook verification process.
//! This includes `hub.mode`, `hub.challenge`, and `hub.verify_token`.
//!
//! # Example
//!
//! A typical query from Facebook for webhook verification might look like:
//! `GET /webhook?hub.verify_token=<your_verify_token>&hub.challenge=CHALLENGE_ACCEPTED&hub.mode=subscribe`
//!
//! The `WebQuery` struct extracts these parameters and validates them against the environment's
//! verification token. If the mode is `subscribe` and the tokens match, it responds with the
//! `hub.challenge` to confirm the subscription; otherwise, it responds with an error.
use actix_web::HttpResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WebQuery {
    #[serde(rename = "hub.mode")]
    hub_mode: Option<String>,
    #[serde(rename = "hub.challenge")]
    hub_challenge: Option<String>,
    #[serde(rename = "hub.verify_token")]
    hub_verify_token: Option<String>,
}

impl WebQuery {
    pub fn get_hub_challenge(&self) -> HttpResponse {
        match (
            self.hub_mode.clone(),
            self.hub_challenge.clone(),
            self.hub_verify_token.clone(),
        ) {
            (Some(hub_mode), Some(hub_challenge), Some(token)) => {
                if hub_mode.eq("subscribe") && std::env::var("VERIFY_TOKEN").unwrap().eq(&token) {
                    HttpResponse::Ok().body(hub_challenge)
                } else {
                    HttpResponse::Unauthorized().body("Token mismatch")
                }
            }
            _ => HttpResponse::Unauthorized().body("Insufficient arguments provided"),
        }
    }
}
