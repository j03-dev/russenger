use url::form_urlencoded;

use crate::Action;
pub use crate::core::data::Data;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Payload {
    path_action: String,
    data: Option<Data>,
}

pub enum ActionPayload {
    Action(Box<dyn Action>),
    PathAction(String),
}

impl Payload {
    pub fn new(action_payload: ActionPayload, data: Option<Data>) -> Self {
        let path_action = match action_payload {
            ActionPayload::Action(action) => action.path(),
            ActionPayload::PathAction(path_action) => path_action,
        };

        Self {
            path_action,
            data,
        }
    }


    pub fn get_path_action(&self) -> &String {
        &self.path_action
    }

    pub fn get_data_to_string(&self) -> String {
        serde_json::to_string(&self.data).unwrap_or_default()
    }

    pub fn to_uri_string(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .append_pair("action", self.get_path_action())
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
            (Some(path_action), Some(value)) => Ok(Self::new(ActionPayload::PathAction(path_action), Some(Data::from(value)))),
            _ => Err("Missing fields in URI".to_string()),
        }
    }
}
