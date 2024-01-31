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
    // Define the type of error that can occur when constructing a FacebookRequest from a request
    type Error = FacebookRequestError<'a>;

    // Define the asynchronous method that constructs a FacebookRequest from a request
    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        // Extract the query string from the request URI
        let query = request
            .uri()
            .query()
            .expect("failed to get query from request");

        // Split the query string into segments and collect them into a vector
        let args: Vec<_> = query.segments().collect();

        // Initialize variables to hold the hub mode, hub challenge, and token
        let mut hub_mode = None;
        let mut hub_challenge = None;
        let mut token = None;

        // Iterate over the query arguments
        for (key, value) in args {
            // Match the key to one of the expected keys and assign the value accordingly
            match key {
                "hub.mode" => hub_mode = Some(value),
                "hub.challenge" => hub_challenge = Some(value),
                "hub.verify_token" => token = Some(value),
                _ => (),  // Ignore any other keys
            }
        }

        // Match the values of the hub mode, hub challenge, and token
        match (hub_mode, hub_challenge, token) {
            // If all values are present
            (Some(hub_mode), Some(hub_challenge), Some(token)) => {
                // If the hub mode is "subscribe" and the token matches the verify token
                if hub_mode.eq("subscribe") && env::var("VERIFY_TOKEN").unwrap().eq(token) {
                    // Return a successful outcome with the hub challenge
                    Outcome::Success(Self(hub_challenge.to_string()))
                } else {
                    // Otherwise, return an error outcome indicating a token mismatch
                    Outcome::Error((
                        Status::Unauthorized,
                        FacebookRequestError::VerificationFailed("Token mismatch"),
                    ))
                }
            }
            // If any values are missing, return an error outcome indicating insufficient arguments
            _ => Outcome::Error((Status::Unauthorized, FacebookRequestError::ArgsNotEnough)),
        }
    }
}
