pub mod button;
pub mod generic;
pub mod get_started;
pub mod media;
pub mod payload;
pub mod persistent_menu;
pub mod quick_replies;
pub mod recipient;
pub mod text;
pub mod data;

use rocket::serde::Serialize;

pub trait ResponseModel: Serialize {
    const END_POINT: &'static str;

    fn get_endpoint(&self) -> &'static str {
        Self::END_POINT
    }
}
