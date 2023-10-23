use crate::query::Query;

pub struct Req<'r> {
    pub user: &'r str,
    pub query: &'r Query,
    pub data: &'r str,
}

impl<'r> Req<'r> {
    pub fn new(user: &'r str, query: &'r Query, data: &'r str) -> Self {
        Self {
            user,
            query,
            data,
        }
    }
}
