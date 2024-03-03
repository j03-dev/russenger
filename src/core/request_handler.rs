use std::env;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

#[derive(Debug)]
pub enum HandleRequestError<'a> {
    ArgsNotEnough,
    HostNotFound,
    VerificationFailed(&'a str),
}

pub struct WebQuery<'w> {
    pub hub_challenge: &'w str,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for WebQuery<'a> {
    type Error = HandleRequestError<'a>;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let query = request
            .uri()
            .query()
            .expect("failed to get query from request");

        let mut hub_mode = None;
        let mut hub_challenge = None;
        let mut token = None;

        for (key, value) in query.segments() {
            match key {
                "hub.mode" => hub_mode = Some(value),
                "hub.challenge" => hub_challenge = Some(value),
                "hub.verify_token" => token = Some(value),
                _ => (),
            }
        }

        match (hub_mode, hub_challenge, token) {
            (Some("subscribe"), Some(hub_challenge), Some(token)) => {
                if env::var("VERIFY_TOKEN").ok() == Some(token.to_string()) {
                    Outcome::Success(Self { hub_challenge })
                } else {
                    Outcome::Error((
                        Status::Unauthorized,
                        HandleRequestError::VerificationFailed("Token mismatch"),
                    ))
                }
            }
            _ => Outcome::Error((Status::Unauthorized, HandleRequestError::ArgsNotEnough)),
        }
    }
}

pub struct WebRequest {
    pub host: String,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for WebRequest {
    type Error = HandleRequestError<'a>;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        match request.host() {
            Some(host) => Outcome::Success(Self {
                host: host.to_string(),
            }),
            None => Outcome::Error((Status::BadRequest, HandleRequestError::HostNotFound)),
        }
    }
}
