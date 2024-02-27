use rocket::serde::{Deserialize, Serialize};

pub type Pagination = [usize; 2];

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct Data {
    value: String,
    pages: Option<Pagination>,
}

const MAX: usize = 500;

trait Verify: ToString {
    fn verify(&self) -> String;
}

impl Verify for String {
    fn verify(&self) -> String {
        if self.len() >= MAX {
            self[..MAX].to_string()
        } else {
            self.clone()
        }
    }
}

impl Data {
    pub fn new<T: Serialize>(value: T, pages: Option<Pagination>) -> Self {
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
        self.pages.unwrap_or([0, 5])
    }
}
