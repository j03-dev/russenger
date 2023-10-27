use crate::Res;
use crate::Data;
use crate::response_models::payload::Payload;
use crate::response_models::quick_replies::{QuickReplie, QuickReplieModel};

type Path = String;

pub enum TypeAction {
	Prev(Path, Data),
	Next(Path, Data)
}

#[rocket::async_trait]
pub trait SendAnotherAction {
    async fn send_id(&self, user: &str, type_action: TypeAction) {
        match type_action {
			TypeAction::Prev(path, mut data) => { 
				Res.send(QuickReplieModel::new(
					user,
					"Action",
					&vec![QuickReplie::new("Prev", "", Payload::new(&path, Some(data.prev_page())))],
				)).await.unwrap();
			},
			TypeAction::Next(path, mut data) => {
				Res.send(QuickReplieModel::new(
					user,
					"Action",
					&vec![QuickReplie::new("Next", "", Payload::new(&path, Some(data.next_page())))],
				)).await.unwrap();
			}
		}
    }
}
