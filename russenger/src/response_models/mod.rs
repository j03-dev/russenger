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
//! use russenger::prelude::*;
//!
//! #[action]
//! async fn index(res: Res, req: Req) -> Result<()> {
//!     let text_model = TextModel::new(&req.user, "Hello, user1!");
//!     res.send(text_model).await?;
//!
//!     Ok(())
//! }
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

use serde::Serialize;

pub trait ResponseModel: Serialize {
    const END_POINT: &'static str;

    fn get_endpoint(&self) -> &'static str {
        Self::END_POINT
    }
}

pub mod data {
    use serde::{Deserialize, Serialize};

    const MAX_VALUE_AUTHORIZED: usize = 500;
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

    trait Verify {
        fn verify(&self) -> String;
    }

    impl Verify for String {
        fn verify(&self) -> String {
            if self.len() >= MAX_VALUE_AUTHORIZED {
                self[..MAX_VALUE_AUTHORIZED].to_string()
            } else {
                self.clone()
            }
        }
    }

    /// The `Data` struct represents a data object with a value and an optional page.
    ///
    /// This struct is used to store and manipulate data. It contains a `value` field, which is a serialized JSON string, and a `page` field, which is an optional `Page` struct.
    ///
    /// # Fields
    ///
    /// * `value`: The value of the data. This is a serialized JSON string.
    /// * `page`: The page of the data. This is an optional `Page` struct.
    ///
    /// # Methods
    ///
    /// * `new`: This method creates a new `Data`. It takes a value and an optional page as arguments, serializes the value into a JSON string, and returns a `Data` with the serialized string and the page.
    /// * `get_value`: This method deserializes the value of the data into a specified type. It returns the deserialized value if it exists, or the default value of the type if it doesn't.
    /// * `get_page`: This method returns the page of the data.
    ///
    /// # Examples
    ///
    /// Creating a new `Data` and using it to get the value and the page:
    ///
    /// ```rust
    /// use russenger::response_models::data::{Data, Page};
    ///
    /// let data = Data::new("value", Some(Page::default()));
    /// let value: String = data.get_value();
    /// let page = data.get_page();
    /// ```
    #[derive(Debug, Default, Clone, Deserialize, Serialize)]
    pub struct Data {
        value: String,
        page: Option<Page>,
    }

    impl Data {
        /// Creates a new `Data`.
        ///
        /// This method takes a value and an optional page as arguments, serializes the value into a JSON string, and returns a `Data` with the serialized string and the page.
        ///
        /// # Arguments
        ///
        /// * `value`: The value of the new data.
        /// * `page`: The page of the new data.
        ///
        /// # Returns
        ///
        /// * `Data`: The created `Data`.
        ///
        pub fn new<T: Serialize>(value: T) -> Self {
            let value = serde_json::to_string(&value).unwrap_or_default().verify();
            Self { value, page: None }
        }

        pub fn new_with_page<T: Serialize>(value: T, page: Option<Page>) -> Self {
            let value = serde_json::to_string(&value).unwrap_or_default().verify();
            Self { value, page }
        }

        /// Deserializes the value of the data into a specified type.
        ///
        /// This method returns the deserialized value if it exists, or the default value of the type if it doesn't.
        ///
        /// # Returns
        ///
        /// * `T`: The deserialized value.
        ///
        /// # Examples
        ///
        /// Deserializing the value of the data into a `String`:
        ///
        /// ```rust
        /// use russenger::response_models::data::{Data, Page};
        ///
        /// let data = Data::new("value"());
        /// let value: String = data.get_value();
        /// ```
        pub fn get_value<T: for<'a> Deserialize<'a> + Default>(&self) -> T {
            serde_json::from_str::<T>(&self.value).unwrap_or_default()
        }

        /// Returns the page of the data.
        ///
        /// # Returns
        ///
        /// * `Option<Page>`: The page of the data.
        ///
        /// # Examples
        ///
        /// Getting the page of the data:
        ///
        /// ```rust
        /// use russenger::response_models::data::{Data, Page};
        ///
        /// let data = Data::new_with_page("value", Some(Page::default()));
        /// let page = data.get_page();
        /// ```
        pub fn get_page(&self) -> Option<Page> {
            self.page.clone()
        }
    }
}
