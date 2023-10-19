use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct Recipient<'r> {
    pub id: &'r str,
}
