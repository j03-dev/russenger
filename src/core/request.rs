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
