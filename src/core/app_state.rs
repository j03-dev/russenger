use crate::query::Query;

#[derive(Clone)]
pub struct AppState {
    pub query: Query,
}

impl AppState {
    pub async fn init() -> Self {
        let query: Query = Query::new().await;
        Self { query }
    }
}
