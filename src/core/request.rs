use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use crate::query::Query;
use crate::Data;

#[derive(Clone)]
pub struct Req {
    pub user: String,
    pub query: Query,
    pub data: Data,
    pub host: String,
}

impl Req {
    pub fn new(user: &str, query: Query, data: Data, host: &str) -> Self {
        Self {
            user: user.into(),
            query,
            data,
            host: host.into(),
        }
    }
}

pub struct RussengerUri {
    pub host: String,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for RussengerUri {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        match request.host() {
            Some(host) => Outcome::Success(Self {
                host: host.to_string(),
            }),
            None => Outcome::Error((Status::BadRequest, ())),
        }
    }
}
