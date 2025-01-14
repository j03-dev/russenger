use super::{
    data::Data,
    payload::Payload,
    quick_replies::{QuickReply, QuickReplyModel},
};

#[allow(non_snake_case)]
pub fn NextModel<'n>(sender: &'n str, data: Data, path: &'n str) -> QuickReplyModel<'n> {
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
