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
pub use constants::{STD_PATH};

pub mod structs;
pub use structs::{localizer::Localizer};
#[cfg(feature = "format")]
pub(crate) use structs::{Member};

mod funcs;
pub use funcs::{finds, finds_all};

mod traits;
pub use traits::*;//{Res, Form, ContFormArgs, FormTo, FormArg};

#[cfg(not(feature = "no_std"))]
use std::collections::HashMap;

#[cfg(not(feature = "no_std"))]
pub use std::fmt::Display;

#[cfg(feature = "no_std")]
pub use core::fmt::Display;

#[cfg(not(feature = "no_std"))]
use enums::standart::res_keeper;

pub mod enums;
pub use enums::serialize_form;

use std::sync::RwLock;

#[macro_use]
extern crate lazy_static;
lazy_static! {
	static ref LANGS : RwLock<Localizer> = RwLock::new(
	#[cfg(not(feature = "no_std"))]
	{Localizer::create().res(res_keeper::new_file_from_path(None)).current_lang(Some("en".to_string())).user(&|x|{x.into_string();})},
	#[cfg(feature = "no_std")]
	{Localizer::create().current_lang(Some("en".to_string()))}
	);
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

#[cfg(feature = "format")]
///returns value from dictionary if it exists otherwise returns second parameter
pub fn get_form_by_key(key : &str, val2 : String, args : &dyn ContFormArgs /*HashMap<String, &dyn Display>*/) -> String
{
	let n = LANGS.read().unwrap();
	n.get_f1(key, args).unwrap_or(val2)
}


///returns Vec of value from dictionary
pub fn get_by_key3(key : &str) -> Vec<String>
{
	LANGS.read().unwrap().get1(key)
}

#[cfg(feature = "format")]
///returns Vec of value from dictionary
pub fn build<'a >(a : &[&'a dyn Display]) -> HashMap<String, &'a dyn Display> {
	a.iter().enumerate().map(|(ind, x)| (format!("ind_{}", ind), *x)).collect()
}

///changes language, dictionary, etc
pub fn change_localizer(f : &dyn Fn(&mut Localizer) -> ())//<enums::res_keeper>
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

