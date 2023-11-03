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
    async fn send_prvnxt(&self, user: &str, (path, data): (Path, Data)) {
        let [start, end] = data.get_page().unwrap_or([0, 5]);

        let mut quick_replies: Vec<QuickReplie> = Vec::new();

        let value: String = data.get_value();

        if start & end > 0 {
            quick_replies.push(QuickReplie::new(
                "Prev",
                "",
                Payload::new(path, Some(Data::new(&value, Some([start - 5, end - 5])))),
            ));
        }

        quick_replies.push(QuickReplie::new(
            "Next",
            "",
            Payload::new(path, Some(Data::new(&value, Some([start + 5, end + 5])))),
        ));

        Res.send(QuickReplieModel::new(user, "Action", &quick_replies))
            .await
            .unwrap();
    }
}
