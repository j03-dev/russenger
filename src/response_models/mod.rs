use rocket::serde::Serialize;

use crate::{Data, Res};

use self::payload::Payload;
use self::quick_replies::{QuickReplie, QuickReplieModel};

pub mod generic;
pub mod media;
pub mod payload;
pub mod quick_replies;
pub mod recipient;
pub mod text;

pub trait GetSender<'r> {
    fn get_sender(&self) -> &'r str;
}

#[rocket::async_trait]
pub trait NextPrevNavigation<'n>: Serialize + GetSender<'n> {
    async fn send_next_prev(&self, path: &str, data: Data) {
        let [start, end] = data.get_page().unwrap_or([0, 5]);
        let mut navigations: Vec<QuickReplie> = Vec::new();

        let value: String = data.get_value();

        if start >= 5 && end >= 5 {
            let prev = QuickReplie::new(
                "Prev",
                "",
                Payload {
                    path_action: path.into(),
                    data: Some(Data::new(&value, Some([start - 5, end - 5]))),
                },
            );
            navigations.push(prev);
        }

        let next = QuickReplie::new(
            "Next",
            "",
            Payload {
                path_action: path.into(),
                data: Some(Data::new(&value, Some([start + 5, end + 5]))),
            },
        );

        navigations.push(next);

        Res.send(QuickReplieModel::new(
            self.get_sender(),
            "Navigation",
            &navigations,
        ))
        .await
        .unwrap();
    }
}
