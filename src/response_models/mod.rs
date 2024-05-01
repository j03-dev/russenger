//! The `response_models` module contains various response models that can be sent to a user.
//!
//! Each response model is represented by a struct that implements the `ResponseModel` trait.
//!
//! # Submodules
//!
//! * `button`: This module contains the `ButtonModel` struct.
//! * `sender_action`: This module contains the `SenderActionModel` struct.
//! * `generic`: This module contains the `GenericTemplateModel` struct.
//! * `get_started`: This module contains the `GetStartedModel` struct.
//! * `media`: This module contains the `MediaModel` struct.
//! * `payload`: This module contains the `PayloadModel` struct.
//! * `persistent_menu`: This module contains the `PersistentMenuModel` struct.
//! * `quick_replies`: This module contains the `QuickRepliesModel` struct.
//! * `recipient`: This module contains the `RecipientModel` struct.
//! * `text`: This module contains the `TextModel` struct.
//!
//! # Traits
//!
//! * `ResponseModel`: This trait is implemented by all response models. It has a `get_endpoint` method that returns the endpoint to which the response should be sent.
//!
//! # Structs
//!
//! * `Page`: This struct represents a page of responses. It has a `next` method that moves to the next page and a `default` method that returns the first page.
//!
//! # Constants
//!
//! * `MAX_VALUE_AUTORIZED`: The maximum value that is authorized.
//! * `MIN_PAGE`: The minimum page number.
//! * `MAX_PAGE`: The maximum page number.
//!
//! # Examples
//!
//! Sending a `TextModel` response:
//!
//! ```rust
//! use crate::response_models::text::TextModel;
//! use crate::core::Res;
//!
//! let res = Res;
//! let text_model = TextModel::new("user1", "Hello, user1!");
//!
//! let send_result = res.send(text_model).await;
//! ```
pub mod button;
pub mod generic;
pub mod get_started;
pub mod media;
pub mod payload;
pub mod persistent_menu;
pub mod quick_replies;
pub mod recipient;
pub mod sender_action;
pub mod text;

use rocket::serde::Serialize;

pub trait ResponseModel: Serialize {
    const END_POINT: &'static str;

    fn get_endpoint(&self) -> &'static str {
        Self::END_POINT
    }
}

pub mod data {
    use rocket::serde::{Deserialize, Serialize};

    const MAX_VALUE_AUTORIZED: usize = 500;
    const MIN_PAGE: usize = 0;
    pub const MAX_PAGE: usize = 10;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Page(pub usize, pub usize);

    impl Page {
        pub fn next(&mut self) {
            self.0 += MAX_PAGE;
            self.1 += MAX_PAGE;
        }
    }

    impl Default for Page {
        fn default() -> Self {
            Self(MIN_PAGE, MAX_PAGE)
        }
    }

    trait Verify: ToString {
        fn verify(&self) -> String;
    }

    impl Verify for String {
        fn verify(&self) -> String {
            if self.len() >= MAX_VALUE_AUTORIZED {
                self[..MAX_VALUE_AUTORIZED].to_string()
            } else {
                self.clone()
            }
        }
    }

    #[derive(Debug, Default, Clone, Deserialize, Serialize)]
    pub struct Data {
        value: String,
        page: Option<Page>,
    }

    impl Data {
        pub fn new<T: Serialize>(value: T, page: Option<Page>) -> Self {
            let value = serde_json::to_string(&value).unwrap_or_default().verify();
            Self { value, page }
        }

        pub fn get_value<T: for<'a> Deserialize<'a> + Default>(&self) -> T {
            serde_json::from_str::<T>(&self.value).unwrap_or_default()
        }

        pub fn get_page(&self) -> Option<Page> {
            self.page.clone()
        }
    }
}
