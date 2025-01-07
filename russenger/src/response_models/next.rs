use super::{
    data::Data,
    payload::Payload,
    quick_replies::{QuickReply, QuickReplyModel},
}; 

pub struct NextModel;

impl NextModel {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'l>(sender: &'l str, data: Data, path: &'l str) -> QuickReplyModel<'l> {
        QuickReplyModel::new(
            sender,
            "Navigation",
            vec![QuickReply::new(
                "Next",
                None,
                Payload::new(path, Some(data.next())),
            )],
        )
    }
}
