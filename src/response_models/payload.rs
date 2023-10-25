use url::form_urlencoded;

use crate::core::data::Data;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Payload {
    action: String,
    data: Option<Data>,
}


impl Payload {
    pub fn new(action: &str, data: Option<Data>) -> Self {
        Self {
            action: action.into(),
            data,
        }
    }

    pub fn get_action(&self) -> &String {
        &self.action
    }

    pub fn get_data_to_string(&self) -> String {
        serde_json::to_string(&self.data).unwrap_or_default()
    }

    pub fn to_uri_string(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .append_pair("action", self.get_action())
            .append_pair("value", &self.get_data_to_string())
            .finish()
    }

    pub fn from_uri_string(uri: &str) -> Result<Self, String> {
        let parsed: Vec<(String, String)> = form_urlencoded::parse(uri.as_bytes())
            .into_owned()
            .collect();

        let mut action = None;
        let mut value = None;

        for (name, val) in parsed {
            match name.as_str() {
                "action" => action = Some(val),
                "value" => value = Some(val),
                _ => return Err(format!("Unknown field in URI: {}", name)),
            }
        }

        match (action, value) {
            (Some(action), Some(value)) => Ok(Self::new(&action, Some(Data::from_str(&value)))),
            _ => Err("Missing fields in URI".to_string()),
        }
    }
}
