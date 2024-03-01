use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use crate::query::Query;
use crate::Data;

#[derive(Clone)]
pub struct Req {
    pub user: String,
    pub query: Query,
    pub data: Data,
    pub uri: String,
}

impl Req {
    pub fn new(user: &str, query: Query, data: Data, uri: &str) -> Self {
        Self {
            user: user.into(),
            query,
            data,
            uri: uri.into(),
        }
    }
}

pub struct RussengerUri {
    pub uri_path: String,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for RussengerUri {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(Self {
            uri_path: request.uri().path().to_string(),
        })
    }
}
