use crate::{
    prelude::{String, ToString},
    structs::LangsDictionary,
};

#[cfg(feature = "std")]
#[derive(Debug, Clone, PartialEq)]
pub enum SerializeForm {
    Toml,
    Json,
    Another(String),
}

#[cfg(not(feature = "std"))]
#[derive(Debug, Clone, PartialEq)]
pub enum SerializeForm {
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
    #[cfg(feature = "std")]
    #[allow(dead_code, unreachable_patterns)]
    pub fn to_str(&self) -> &str {
        match self {
            Self::Toml => "toml",
            Self::Json => "json",
            Self::Another(ref s) => s,
        }
    }
    #[cfg(not(feature = "std"))]
    #[allow(dead_code, unreachable_patterns)]
    pub fn to_str(&self) -> &str {
        match self {
            Self::Json => "json",
            Self::Another(ref s) => s,
        }
    }
    #[cfg(feature = "std")]
    pub fn create(&self, string: &str) -> Option<LangsDictionary> {
        match self {
            SerializeForm::Toml => toml::from_str(string).ok(),
            SerializeForm::Json => serde_json::from_str(string).ok(),
            SerializeForm::Another(_) => None,
        }
    }
    #[cfg(not(feature = "std"))]
    pub fn create(&self, string: &str) -> Option<LangsDictionary> {
        match self {
            SerializeForm::Json => serde_json::from_str(string).ok(),
            SerializeForm::Another(_) => None,
        }
    }
}

impl crate::prelude::FromStr for SerializeForm {
    type Err = ();
    #[cfg(feature = "std")]
    fn from_str(a: &str) -> Result<SerializeForm, ()> {
        match a {
            "toml" => Ok(SerializeForm::Toml),
            "json" => Ok(SerializeForm::Json),
            s => Ok(SerializeForm::Another(s.to_string())),
        }
    }
    #[cfg(not(feature = "std"))]
    fn from_str(a: &str) -> Result<SerializeForm, ()> {
        match a {
            "json" => Ok(SerializeForm::Json),
            s => Ok(SerializeForm::Another(s.to_string())),
        }
    }
}

impl Default for SerializeForm {
    #[cfg(feature = "std")]
    fn default() -> Self {
        SerializeForm::Toml
    }
    #[cfg(not(feature = "std"))]
    fn default() -> Self {
        SerializeForm::Json
    }
}
