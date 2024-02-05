use crate::Data;
use crate::query::Query;

#[derive(Clone)]
pub struct Req {
    pub user: String,
    pub query: Query,
    pub data: Data,
}

impl Req {
    pub fn new(user: &str, query: Query, data: Data) -> Self {
        Self {
            user: user.into(),
            query,
            data,
        }
    }
}
