use crate::{
    prelude::{String, ToString},
    structs::LangsDictionary,
};

#[derive(Debug, Clone, PartialEq)]
pub enum SerializeForm {
    Toml,
    Json,
    Another(String),
}
impl SerializeForm {
    /*#[allow(dead_code)]
    pub fn from_str(a: &str) -> Self {
        match a {
            "toml" => Self::Toml,
            "json" => Self::Json,
            s => Self::Another(s.to_string()),
        }
    }*/
    #[allow(dead_code, unreachable_patterns)]
    pub fn to_str(&self) -> &str {
        match self {
            Self::Toml => "toml",
            Self::Json => "json",
            Self::Another(ref s) => s,
        }
    }
    pub fn create(&self, string: &str) -> Option<LangsDictionary> {
        match self {
            SerializeForm::Toml => toml::from_str(string).ok(),
            SerializeForm::Json => serde_json::from_str(string).ok(),
            SerializeForm::Another(_) => None,
        }
    }
}

impl crate::prelude::FromStr for SerializeForm {
    type Err = ();
    fn from_str(a: &str) -> Result<SerializeForm, ()> {
        match a {
            "toml" => Ok(SerializeForm::Toml),
            "json" => Ok(SerializeForm::Json),
            s => Ok(SerializeForm::Another(s.to_string())),
        }
    }
}

impl Default for SerializeForm {
    fn default() -> Self {
        Self::Toml
    }
}
