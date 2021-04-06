use crate::prelude::{String, ToString};

pub struct Properties {
    pub standart_lang: String,
    pub current_lang: Option<String>,
}
impl Properties {
    const DEFAULT_LANG: &'static str = "en";
    pub fn new() -> Self {
        Self {
            standart_lang: Self::DEFAULT_LANG.to_string(),
            current_lang: None,
        }
    }
    pub fn set_standart_lang(&mut self, val: String) {
        self.standart_lang = val;
    }
    pub fn set_current_lang(&mut self, val: Option<String>) {
        self.current_lang = val;
    }

    pub fn get_standart_lang(&self) -> &String {
        &self.standart_lang
    }
    pub fn get_current_lang(&self) -> &Option<String> {
        &self.current_lang
    }
}

impl Default for Properties {
    fn default() -> Self {
        Self::new()
    }
}
