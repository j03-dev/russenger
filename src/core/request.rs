use super::Data;
use crate::query::Query;

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
            user: user.to_owned(),
            query,
            data,
            host: host.to_owned(),
        }
    }
}
