use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Recipient<'r> {
    pub id: &'r str,
}
