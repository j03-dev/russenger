use url::form_urlencoded;

#[derive(Debug, Clone, PartialEq)]
pub struct Payload {
    action: String,
    key: String,
    value: String,
}

impl Payload {
    pub fn new(action: &str, key: &str, value: &str) -> Self {
        Self {
            action: action.into(),
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn get_action(&self) -> &String {
        &self.action
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn to_uri_string(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .append_pair("action", self.get_action())
            .append_pair("key", self.get_key())
            .append_pair("value", self.get_value())
            .finish()
    }

    pub fn from_uri_string(uri: &str) -> Result<Self, String> {
        let parsed: Vec<(String, String)> = form_urlencoded::parse(uri.as_bytes())
            .into_owned()
            .collect();

        let mut action = None;
        let mut key = None;
        let mut value = None;

        for (name, val) in parsed {
            match &name[..] {
                "action" => action = Some(val),
                "key" => key = Some(val),
                "value" => value = Some(val),
                _ => return Err(format!("Unknown field in URI: {}", name)),
            }
        }

        match (action, key, value) {
            (Some(action), Some(key), Some(value)) => Ok(Self::new(&action, &key, &value)),
            _ => Err("Missing fields in URI".to_string()),
        }
    }
}
