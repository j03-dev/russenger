use rocket::serde::Serialize;

use crate::{Data, Res};

use self::payload::{ActionPayload, Payload};
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
        if let Some([start, end]) = data.get_page() {
            let mut navigations: Vec<QuickReplie> = Vec::new();
            let next = QuickReplie::new(
                "Next",
                "",
                Payload::new(
                    ActionPayload::Path(path.into()),
                    Some(Data::new(
                        data.get_value::<String>(),
                        Some([start + 5, end + 5]),
                    )),
                ),
            );

            navigations.push(next);

            if start >= 5 && end >= 5 {
                let prev = QuickReplie::new(
                    "Prev",
                    "",
                    Payload::new(
                        ActionPayload::Path(path.into()),
                        Some(Data::new(
                            data.get_value::<String>(),
                            Some([start - 5, end - 5]),
                        )),
                    ),
                );
                navigations.push(prev);
            }

            Res.send(QuickReplieModel::new(
                self.get_sender(),
                "Navigation",
                &navigations,
            ))
            .await
            .unwrap();
        }
    }
}
