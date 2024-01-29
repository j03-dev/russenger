use rocket::serde::{Deserialize, Serialize};

pub type Pagination = [usize; 2];

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct Data {
    value: String,
    pages: Option<Pagination>,
}

impl Data {
    pub fn new<T: Serialize>(value: T, pages: Option<Pagination>) -> Self {
        let value = serde_json::to_string(&value).unwrap_or_default();
        Self { value, pages }
    }

    pub fn from<T: ToString>(data: T) -> Self {
        serde_json::from_str(&data.to_string()).unwrap_or_default()
    }

    pub fn get_value<T: for<'a> Deserialize<'a> + Default>(&self) -> T {
        serde_json::from_str::<T>(&self.value).unwrap_or_default()
    }

    pub fn get_page(&self) -> Option<Pagination> {
        self.pages
    }
}
