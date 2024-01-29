use url::form_urlencoded;

pub use crate::core::data::Data;
use crate::Action;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Payload {
    pub path: String,
    pub data: Option<Data>,
}

impl Payload {
    pub fn new(action: Box<dyn Action>, data: Option<Data>) -> Self {
        Self {
            path: action.path(),
            data,
        }
    }
    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_data_to_string(&self) -> String {
        serde_json::to_string(&self.data).unwrap_or_default()
    }

    pub fn to_uri_string(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .append_pair("action", self.get_path())
            .append_pair("value", &self.get_data_to_string())
            .finish()
    }

    pub fn from_uri_string(uri: &str) -> Result<Self, String> {
        let parsed: Vec<(String, String)> = form_urlencoded::parse(uri.as_bytes())
            .into_owned()
            .collect();

        let mut path = None;
        let mut value = None;

        for (name, val) in parsed {
            match name.as_str() {
                "action" => path = Some(val),
                "value" => value = Some(val),
                _ => return Err(format!("Unknown field in URI: {}", name)),
            }
        }

        match (path, value) {
            (Some(path), Some(value)) => {
                let data = Some(Data::from_string(value));
                Ok(Self { path, data })
            }
            _ => Err("Missing fields in URI".to_string()),
        }
    }
}
