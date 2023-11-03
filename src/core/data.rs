use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct Data {
    value: String,
    pages: Option<[usize; 2]>,
}

impl Data {
    pub fn new<T: Serialize>(value: T, pages: Option<[usize; 2]>) -> Self {
        let value = serde_json::to_string(&value).unwrap_or_default();
        Self { value, pages }
    }

    pub fn from<T: ToString>(data: T) -> Data {
        serde_json::from_str(&data.to_string()).unwrap_or_default()
    }

    pub fn get_value<T: for<'a> Deserialize<'a> + Default>(&self) -> T {
        serde_json::from_str::<T>(&self.value).unwrap_or_default()
    }

    pub fn get_page(&self) -> Option<[usize; 2]> {
        self.pages
    }
}
