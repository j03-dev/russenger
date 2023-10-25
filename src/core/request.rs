use crate::core::data::Data;
use crate::query::Query;

pub struct Req<'r> {
    pub user: &'r str,
    pub query: &'r Query,
    pub data: Data,
}

impl<'r> Req<'r> {
    pub fn new(user: &'r str, query: &'r Query, data: Data) -> Self {
        Self { user, query, data }
    }
}
