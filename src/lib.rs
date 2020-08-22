//!Simple library for localization.

//! # Example:
//! ```
//! use localizer::*;
//! const langs_file : &str = "
//! [langs.ru]
//! \"ex1\" = \"ПРН\"
//!
//! [langs.en]
//! \"ex1\" = \"IKA\"";
//! localizer::set_loc_string(langs_file);
//! //localizator::set_localizer(Localizer::create().file(None).current_lang(Some("ru".to_string())));
//! localizer::change_localizer(&|x|{x.set_current_lang(Some("ru".to_string()));});
//! println!("{:?}", localizer::get_by_key(&"ex1"));
//! let loc = Localizer::create().file(None).current_lang(Some("ru".to_string()));
//! println!("{:?}", loc.get(&"ex1"));
//! ```

pub mod constants;
pub use constants::{STD_PATH};

pub mod structs;
pub use structs::{Localizer};



pub mod enums;

use std::sync::RwLock;
#[macro_use]
extern crate lazy_static;
lazy_static! {
	static ref LANGS : RwLock<Localizer> = RwLock::new(Localizer::create().file(None).current_lang(Some("en".to_string())).user(&|x|{x.into_string();}));
}

///returns value from dictionary if it exists otherwise returns None
pub fn get_by_key(key : &str) -> Option<String>
{
	LANGS.read().unwrap().get(key)//LANGS.read()/*.unwrap()*/.ok()?.get(key)//
}
///returns value from dictionary if it exists otherwise returns key
pub fn get_by_key1(val0 : String) -> String
{
	get_by_key(&val0).unwrap_or(val0)
}
///returns value from dictionary if it exists otherwise returns second parameter
pub fn get_by_key2(key : &str, val2 : String) -> String
{
	get_by_key(key).unwrap_or(val2)
}
///changes language, dictionary, etc
pub fn change_localizer(f : &dyn Fn(&mut Localizer) -> ())
{
	f(&mut LANGS.write().unwrap());
}
///changes string
pub fn set_loc_string(string : &str)
{
	change_localizer(&|x|{x.set_string(Some(string.to_string()));});
}

pub fn into_string()
{
	change_localizer(&|x|{x.into_string();});
}

pub fn set_localizer(new : Localizer)
{
	*LANGS.write().unwrap() = new;
}
