use crate::prelude::{
    fmt::{self, Debug, Display},
    Into, String,
};
pub enum Error {
    String(String),
    Unknown,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::String(ref s) => write!(f, "Error: {}", s),
            Error::Unknown => write!(f, "Unknown Error"),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::String(ref s) => write!(f, "Error: {:?}", s),
            Error::Unknown => write!(f, "Unknown Error"),
        }
        /*f.debug_struct("Error")
        .field("x", &self.x)
        .field("y", &self.y)
        .finish()*/
    }
}

impl Into<String> for Error {
    fn into(self) -> String {
        match self {
            Error::String(s) => s,
            _ => panic!("Unknown Error"),
        }
    }
}

impl Into<Error> for String {
    fn into(self) -> Error {
        Error::String(self)
    }
}

impl Into<Error> for &str {
    fn into(self) -> Error {
        Error::String(self.to_owned())
    }
}
