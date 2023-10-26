use crate::core::response::Res;
use crate::response_models::payload::Payload;
use crate::response_models::quick_replies::{QuickReplie, QuickReplieModel};

#[rocket::async_trait]
pub trait NextAction {
    async fn next(&self, user: &str, payload: Payload) {
        Res.send(QuickReplieModel::new(
            user,
            "Action",
            &vec![QuickReplie::new("Next", "", payload)],
        ))
        .await
        .unwrap();
    }
}
