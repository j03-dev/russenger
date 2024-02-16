use std::env;

use rocket::http::Status;
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

// Define the FacebookRequestError enum
// This enum represents the possible errors that can occur when processing a Facebook request
#[derive(Debug)]
pub enum FacebookRequestError<'a> {
    VerificationFailed(&'a str),
    ArgsNotEnough,
}

// Define the FacebookRequest struct
// This struct represents a Facebook request, and contains a single public String field
pub struct FacebookRequest(pub String);

// Implement the FromRequest trait for the FacebookRequest struct
// This trait allows a FacebookRequest to be constructed from a Rocket request
#[rocket::async_trait]
impl<'a> FromRequest<'a> for FacebookRequest {
    type Error = FacebookRequestError<'a>;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let query = request
            .uri()
            .query()
            .expect("failed to get query from request");

        let args: Vec<_> = query.segments().collect();

        let mut hub_mode = None;
        let mut hub_challenge = None;
        let mut token = None;

        for (key, value) in args {
            match key {
                "hub.mode" => hub_mode = Some(value),
                "hub.challenge" => hub_challenge = Some(value),
                "hub.verify_token" => token = Some(value),
                _ => (),
            }
        }

        match (hub_mode, hub_challenge, token) {
            (Some(hub_mode), Some(hub_challenge), Some(token)) => {
                if hub_mode.eq("subscribe") && env::var("VERIFY_TOKEN").unwrap().eq(token) {
                    Outcome::Success(Self(hub_challenge.to_string()))
                } else {
                    Outcome::Error((
                        Status::Unauthorized,
                        FacebookRequestError::VerificationFailed("Token mismatch"),
                    ))
                }
            }
            _ => Outcome::Error((Status::Unauthorized, FacebookRequestError::ArgsNotEnough)),
        }
    }
}
