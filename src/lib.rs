#![cfg_attr(not(feature = "std"), no_std)]
//!Simple library for localization.

//!# no unsafe code
//! # Example:
//! ```
//! use localizer::*;
//! const langs_file : &str = "
//! [langs.ru]
//! \"ex1\" = \"ПРН{}\"
//!
//! [langs.en]
//! \"ex1\" = \"IKA{}\"";
//! localizer::set_loc_string(langs_file);
//! //localizator::set_localizer(Localizer::create().file(None).current_lang(Some("ru".to_string())));
//! localizer::change_localizer(&|x|{x.set_current_lang(Some("ru".to_string()));});
//! println!("{:?}", localizer::get_by_key(&"ex1"));
//! let loc = Localizer::create().file(None).current_lang(Some("ru".to_string()));
//! println!("{:?}", loc.get(&"ex1"));
//! ```
pub mod constants;

pub mod structs;
pub use structs::localizer::Localizer;

pub mod enums;

pub mod prelude;
pub use prelude::*;

pub mod error;
pub use error::Error;

#[cfg(feature = "format")]
pub mod formatter;
//#[cfg(feature = "format")]
//pub use form::*;

pub mod traits;
pub use traits::{FnUser};

impl<T> traits::FnUser for T {}

//#[cfg(not(feature = "no_std"))]
//use enums::standart::res_keeper;
