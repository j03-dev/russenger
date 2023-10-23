use url::form_urlencoded;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Payload {
    action: String,
    value: Option<String>,
}

impl Payload {
    pub fn new(action: &str, value: Option<String>) -> Self {
        Self {
            action: action.into(),
            value,
        }
    }

    pub fn get_action(&self) -> &String {
        &self.action
    }

    pub fn get_value(&self) -> String {
        self.value.clone().unwrap_or("none".to_string())
    }

    pub fn to_uri_string(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .append_pair("action", self.get_action())
            .append_pair("value", &self.get_value())
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
            (Some(action), Some(value)) => Ok(Self::new(&action, Some(value))),
            _ => Err("Missing fields in URI".to_string()),
        }
    }
}
