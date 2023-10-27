use crate::response_models::payload::Payload;
use crate::response_models::quick_replies::{QuickReplie, QuickReplieModel};
use crate::Data;
use crate::Res;

type Path = String;

#[rocket::async_trait]
pub trait SendAnotherAction {
    async fn send_prvnxt(&self, user: &str, (path, mut data): (Path, Data)) {
        Res.send(QuickReplieModel::new(
            user,
            "Action",
            &vec![
                QuickReplie::new("Prev", "", Payload::new(&path, Some(data.prev_page()))),
                QuickReplie::new("Next", "", Payload::new(&path, Some(data.next_page()))),
            ],
        ))
        .await
        .unwrap();
    }
}
