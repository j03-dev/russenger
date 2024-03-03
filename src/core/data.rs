use rocket::serde::{Deserialize, Serialize};

pub type Pagination = Option<[usize; 2]>;

const MAX_VALUE_AUTORIZED: usize = 500;

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

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct Data {
    value: String,
    pages: Pagination,
}

impl Data {
    pub fn new<T: Serialize>(value: T, pages: Pagination) -> Self {
        let value = serde_json::to_string(&value).unwrap_or_default().verify();
        Self { value, pages }
    }

    pub fn from_string<T: ToString>(s: T) -> Self {
        serde_json::from_str(&s.to_string()).unwrap_or_default()
    }

    pub fn get_value<T: for<'a> Deserialize<'a> + Default>(&self) -> T {
        serde_json::from_str::<T>(&self.value).unwrap_or_default()
    }

    pub fn get_page(&self) -> Pagination {
        self.pages
    }
}
