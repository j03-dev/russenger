use std::str::FromStr;

use rocket::serde::{Deserialize, Serialize};

use super::data::Data;
use crate::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    path: String,
    data: Option<Data>,
}

impl Payload {
    pub fn new<A: Action>(action: A, data: Option<Data>) -> Self {
        Self {
            path: action.path(),
            data,
        }
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_data(&self) -> Data {
        self.data.clone().unwrap_or_default()
    }
}

impl FromStr for Payload {
    type Err = String;

    fn from_str(payload: &str) -> Result<Self, String> {
        serde_json::from_str(payload).map_err(|err| err.to_string())
    }
}

impl ToString for Payload {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl Default for Payload {
    fn default() -> Self {
        Payload {
            path: "Main".to_owned(),
            data: None,
        }
    }
}
