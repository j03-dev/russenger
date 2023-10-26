use crate::core::response::Res;
use crate::response_models::payload::Payload;
use crate::response_models::quick_replies::{QuickReplie, QuickReplieModel};
use crate::Data;

pub struct Next {
    path: String,
    value: String,
    pages: [usize; 2],
}

impl Next {
    pub fn new(path: &str, value: &str, pages: [usize; 2]) -> Self {
        Self {
            path: path.into(),
            value: value.into(),
            pages,
        }
    }
}

#[rocket::async_trait]
pub trait NextAction {
    async fn next(&self, user: &str, next: Next) {
        Res.send(QuickReplieModel::new(
            user,
            "Next",
            &vec![QuickReplie::new(
                "Next",
                "",
                Payload::new(&next.path, Some(Data::new(&next.value, Some(next.pages)))),
            )],
        ))
        .await
        .unwrap();
    }
}
