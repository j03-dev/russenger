use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Recipient<'r> {
    pub id: &'r str,
}
