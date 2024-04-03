use rocket::serde::{json::Value, Serialize};

use crate::button::Button;

#[derive(Serialize)]
struct Menu<'m> {
    locale: &'m str,
    composer_input_disabled: bool,
    call_to_actions: Vec<Value>,
}

#[derive(Serialize)]
pub struct PersistentMenu<'p> {
    persistent_menu: Vec<Menu<'p>>,
}

impl<'p> PersistentMenu<'p> {
    pub fn new(buttons: Vec<Button>) -> Self {
        let buttons: Vec<_> = buttons.iter().map(|btn| btn.to_value()).collect();

        Self {
            persistent_menu: vec![Menu {
                locale: "default",
                composer_input_disabled: false,
                call_to_actions: buttons,
            }],
        }
    }
}
