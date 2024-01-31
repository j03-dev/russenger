use rocket::serde::{Deserialize, Serialize};

// Define a type alias for Pagination
pub type Pagination = [usize; 2];

// The Data struct represents the data associated with an action.
// It contains a value and optional pagination information.
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct Data {
    value: String,  // The value of the data
    pages: Option<Pagination>,  // The pagination information
}

impl Data {
    // Constructs a new Data with the given value and pagination information.
    // The value is serialized to a JSON string.
    pub fn new<T: Serialize>(value: T, pages: Option<Pagination>) -> Self {
        let value = serde_json::to_string(&value).unwrap_or_default();
        Self { value, pages }
    }

    // Constructs a Data from a string.
    // The string is deserialized from a JSON string.
    pub fn from_string<T: ToString>(s: T) -> Self {
        serde_json::from_str(&s.to_string()).unwrap_or_default()
    }

    // Returns the value of the Data.
    // The value is deserialized from a JSON string.
    pub fn get_value<T: for<'a> Deserialize<'a> + Default>(&self) -> T {
        serde_json::from_str::<T>(&self.value).unwrap_or_default()
    }

    // Returns the pagination information of the Data.
    pub fn get_page(&self) -> Option<Pagination> {
        self.pages
    }
}
