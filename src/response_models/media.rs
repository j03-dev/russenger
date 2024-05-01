use rocket::serde::Serialize;

use super::recipient::Recipient;
use super::ResponseModel;

#[derive(Serialize)]
struct MediaPayload<'p> {
    url: &'p str,
    is_reusable: bool,
}

#[derive(Serialize)]
struct MediaAttachment<'a> {
    #[serde(rename = "type")]
    r#type: &'a str,
    payload: MediaPayload<'a>,
}

#[derive(Serialize)]
struct Attachment<'s> {
    attachment: MediaAttachment<'s>,
}

/// `MediaModel` is used to send media files such as images and videos to the recipient via a Facebook URL.
///
/// The `MediaModel` struct contains the following fields:
/// - `messaging_type`: A string that specifies the type of messaging. For media messages, this is always "RESPONSE".
/// - `recipient`: A `Recipient` struct that specifies the recipient of the media file.
/// - `message`: An `Attachment` struct that contains the type of the media file and the Facebook URL of the media file.
///
/// This model does not allow any external URLs, only those on Facebook.
///
/// # Methods
///
/// * `new(sender: &'m str, media_type: &'m str, url: &'m str) -> Self` - Creates a new `MediaModel` instance.
///
/// # Examples
/// 
/// Sending a video from static dir
/// ```rust
/// use russenger::prelude::*;
/// 
/// create_action!(SendFileFromStaticDir, |res: Res, req: Req| async move {
///     let text = TextModel::new(&req.user, "Sending file... Please wait!");
///     res.send(text).await;
///     let url = format!("{host}/video.mp4", host = req.host);
///     res.send(MediaModel::new(&req.user, "video", &url)).await
/// });
/// ```
///
/// Sending a media file:
///
/// ```rust
/// use russenger::response_models::media::MediaModel;
/// let message = MediaModel::new("sender_id", "image", "https://cdn.pixabay.com/photo/2017/06/28/10/53/board-2450236_960_720.jpg");
/// ```
///
/// [Facebook Documentation](https://developers.facebook.com/docs/messenger-platform/send-messages/template/media)
#[derive(Serialize)]
pub struct MediaModel<'m> {
    messaging_type: &'m str,
    recipient: Recipient<'m>,
    message: Attachment<'m>,
}

impl<'m> MediaModel<'m> {
    /// Creates a new `MediaModel`.
    ///
    /// This method allows you to send a media file to the given recipient via a Facebook URL.
    ///
    /// # Arguments
    ///
    /// * `sender` - A string slice that holds the ID of the recipient. This is the unique identifier for the user or page that will receive the media file.
    /// * `media_type` - A string slice that holds the type of the media file. This should be either "image" or "video".
    /// * `url` - A string slice that holds the Facebook URL of the media file. To get the Facebook URL of an image or video, click on the image or video thumbnail to open the full-size view and copy the URL address from your browser's address bar.
    ///
    /// # Returns
    ///
    /// This method returns a `MediaModel` instance with the `messaging_type` field set to "RESPONSE", the `recipient` field set to the provided recipient ID, and the `message` field set to the provided media type and Facebook URL.
    ///
    /// # Example
    ///
    /// ```rust
    /// use russenger::response_models::media::MediaModel;
    /// let message = MediaModel::new("sender_id", "image", "https://cdn.pixabay.com/photo/2017/06/28/10/53/board-2450236_960_720.jpg");
    /// ```
    ///
    /// [Facebook Documentation](https://developers.facebook.com/docs/messenger-platform/send-messages/template/media)
    pub fn new(sender: &'m str, media_type: &'m str, url: &'m str) -> Self {
        Self {
            messaging_type: "RESPONSE",
            recipient: Recipient { id: sender },
            message: Attachment {
                attachment: MediaAttachment {
                    r#type: media_type,
                    payload: MediaPayload {
                        url,
                        is_reusable: true,
                    },
                },
            },
        }
    }
}

impl ResponseModel for MediaModel<'_> {
    const END_POINT: &'static str = "messages";
}
