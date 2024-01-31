use url::form_urlencoded;

pub use crate::core::data::Data;
use crate::Action;

// The Payload struct represents the payload of a request.
// It contains a path and optional data.
#[derive(Debug, Default, Clone)]
pub struct Payload {
    pub path: String,  // The path of the action
    pub data: Option<Data>,  // The data associated with the action
}

impl Payload {
    // Constructs a new Payload with the given action and data.
    pub fn new<A: Action>(action: A, data: Option<Data>) -> Self {
        Self {
            path: action.path(),  // Set the path to the action's path
            data,  // Set the data to the given data
        }
    }

    // Returns a reference to the path of the Payload.
    pub fn get_path(&self) -> &String {
        &self.path
    }

    // Converts the data of the Payload to a JSON string.
    // If the data cannot be converted to a JSON string, returns an empty string.
    pub fn get_data_to_string(&self) -> String {
        serde_json::to_string(&self.data).unwrap_or_default()
    }

    // Converts the Payload to a URI-encoded string.
    pub fn to_uri_string(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .append_pair("action", self.get_path())  // Add the path as the "action" parameter
            .append_pair("value", &self.get_data_to_string())  // Add the data as the "value" parameter
            .finish()
    }

    // Constructs a Payload from a URI-encoded string.
    // If the string contains an unknown field, returns an error.
    pub fn from_uri_string(uri: &str) -> Result<Self, String> {
        let parsed: Vec<(String, String)> = form_urlencoded::parse(uri.as_bytes())
            .into_owned()
            .collect();

        let mut path = None;
        let mut value = None;

        for (name, val) in parsed {
            match name.as_str() {
                "action" => path = Some(val),  // Set the path to the value of the "action" parameter
                "value" => value = Some(val),  // Set the value to the value of the "value" parameter
                _ => return Err(format!("Unknown field in URI: {}", name)),  // Return an error if an unknown field is encountered
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
