use dotenv::dotenv;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use std::{collections::HashMap, env};

pub mod messages;

#[derive(Debug)]
pub enum FbRequestError {
    VerificationFailed,
    ArgsNotEnough,
}

pub struct FacebookRequest(pub String);

#[rocket::async_trait]
impl<'a> FromRequest<'a> for FacebookRequest {
    type Error = FbRequestError;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let query = request
            .uri()
            .query()
            .expect("failed to get query from request");
        let args: Vec<_> = query.segments().collect();

        let mut hash_args: HashMap<&str, &str> = HashMap::new();
        for (key, value) in &args {
            hash_args.insert(key, value);
        }
        match args.len() {
            3 => {
                let hub_mode = *hash_args.get(&"hub.mode").unwrap_or(&"none");
                let hub_challenge = *hash_args.get(&"hub.challenge").unwrap_or(&"none");
                let token = *hash_args.get(&"hub.verify_token").unwrap_or(&"none");
                dotenv().ok();

                if hub_mode.eq("subscribe") && env::var("VERIFY_TOKEN").unwrap().eq(token) {
                    Outcome::Success(Self(hub_challenge.to_string()))
                } else {
                    Outcome::Failure((Status::Unauthorized, FbRequestError::VerificationFailed))
                }
            }
            _ => Outcome::Failure((Status::Unauthorized, FbRequestError::ArgsNotEnough)),
        }
    }
}
