pub mod generic;
pub mod media;
pub mod payload;
pub mod quick_replies;
pub mod recipient;
pub mod text;

use crate::Data;
use crate::Res;
use payload::Payload;
use quick_replies::{QuickReplie, QuickReplieModel};

type Path = &'static str;

#[rocket::async_trait]
pub trait SendAnotherAction {
    async fn send_prvnxt(&self, user: &str, (path, mut data): (Path, Data)) {
        Res.send(QuickReplieModel::new(
            user,
            "Action",
            &vec![
                QuickReplie::new("Prev", "", Payload::new(path, Some(data.prev_page()))),
                QuickReplie::new("Next", "", Payload::new(path, Some(data.next_page()))),
            ],
        ))
        .await
        .unwrap();
    }
}
