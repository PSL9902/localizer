#![allow(dead_code)]
use std::fs::File;


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum serialize_form
{
	toml,
	json,
}
impl serialize_form {
	#[allow(dead_code)]
	pub fn from_str(a : &str) -> Self {
		match a {
			"toml" => Self::toml,
			"json" => Self::json,
			_ => panic!(),
		}
	}
	#[allow(dead_code, unreachable_patterns)]
	pub fn to_str(&self) -> &str {
		match self {
			Self::toml => "toml",
			Self::json => "json",
			//_ => unimplemented!(),
		}
	}
	pub fn get(&self, string : &String, key : &str, current_lang : &Option<String>) -> Option<String> {
		match self {
			serialize_form::toml => {
						use toml::{de::{Deserializer}, value::Value};
						use serde::de::Deserialize;
						
						let d = Value::deserialize(&mut Deserializer::new(&string)).unwrap();
						match d.type_str()
						{
							"table" => (),
							#[allow(unused_variables)]
							x => {
									#[cfg(debug)]
									dbg!(x);
									#[cfg(not(debug))]
									unimplemented!();
								}
						}
						let table = d.as_table()?;
						
						let langs = table.get("langs")?.as_table()?;
						
						let lang = match current_lang {
							Some(ref x) if langs.contains_key(x.as_str()) => x.to_string(),
							_ => {
										if langs.contains_key("en")
										{"en"}
										else
										{langs.keys().next()?}
									}.to_string(),
						};
						
						Some(langs.get(&lang)?.get(key)?.as_str()?.to_string())
					}
			serialize_form::json => {
						use json::{parse, JsonValue, object::Object};
						
						let d = parse(&string).ok()?;
						
						if !d.is_object()
						{
							#[cfg(debug)]
							dbg!(d);
							#[cfg(not(debug))]
							unimplemented!();
						}
						fn as_table(var : &JsonValue) -> Option<&Object>
						{
							match var {
								JsonValue::Object(table) => Some(table),
								_ => return None,
							}
						}
						let table = match d {
							JsonValue::Object(table) => table,
							_ => return None,
						};
						
						let langs = as_table(table.get("langs")?)?;
						
						let lang = match current_lang {
							Some(ref x) if langs.get(x.as_str()).is_some() => x.to_string(),
							_ => {
										if langs.get("en").is_some()
										{"en"}
										else
										{langs.iter().next()?.0}
									}.to_string(),
						};
						Some(as_table(langs.get(&lang)?)?.get(key)?.as_str()?.to_string())
					}
			//_ /*| serialize_form::json*/ => { unimplemented!() }
		}
	}
	pub fn get1(&self, string : &String, key : &str) -> Vec<String> {
		match self {
			serialize_form::toml => {
						use toml::{de::{Deserializer}, value::Value};
						use serde::de::Deserialize;
						
						let d = Value::deserialize(&mut Deserializer::new(&string)).unwrap();
						let table = match d.type_str()
						{
							"table" => d.as_table().unwrap(),
							#[allow(unused_variables)]
							x => {
									#[cfg(debug)]
									dbg!(x);
									#[cfg(not(debug))]
									unimplemented!();
								}
						};
						
						let langs = table.get("langs").unwrap().as_table().unwrap();
						langs.iter().filter_map(|x| {
							match x.1.as_table().unwrap().get(key) {
								Some(x) => Some(x.to_string()),
								_ => None
							}}).collect()
					}
			serialize_form::json => {
						use json::{parse, JsonValue, object::Object};
						
						let d = parse(&string).unwrap();
						
						if !d.is_object()
						{
							#[cfg(debug)]
							dbg!(d);
							#[cfg(not(debug))]
							unimplemented!();
						}
						fn as_table(var : &JsonValue) -> Option<&Object>
						{
							match var {
								JsonValue::Object(table) => Some(table),
								_ => return None,
							}
						}
						let table = /*match d {
							JsonValue::Object(table) => table,
							_ => return None,
						}*/as_table(&d).unwrap();
						
						as_table(table.get("langs").unwrap()).unwrap().iter().filter_map(|x|
						{
							match as_table(x.1).unwrap().get(key) {
								Some(x) => Some(x.to_string()),
								_ => None,
							}
						}).collect()
						
					}
			//_ /*| serialize_form::json*/ => { unimplemented!() }
		}
	}
}

impl Default for serialize_form {
	fn default() -> Self { Self::toml }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum res_keeper
{
	file(File),
	string(String),
	None,
}
impl res_keeper {
	pub fn as_str(&self) -> &str {match self{Self::file(_) => "file", Self::string(_) => "string", Self::None => "None"}}
	pub fn new_file(res : Option<File>) -> Self {
		match res {
			Some(file) => Self::file(file),
			None => Self::None,
		}
	}
	pub fn new_string(res : Option<String>) -> Self {
		match res {
			Some(string) => Self::string(string),
			None => Self::None,
		}
	}
	pub fn set_file(&mut self, res : Option<File>) {
		match res {
			Some(file) => {*self = Self::file(file);}
			None => {*self = Self::None;}
		}
	}
	pub fn set_string(&mut self, res : Option<String>) {
		match res {
			Some(string) => {*self = Self::string(string);}
			None => {*self = Self::None;}
		}
	}
	pub fn get_file(self) -> Option<File> {
		match self {
			Self::file(file) => Some(file),
			_ => None
		}
	}
	pub fn get_string(self) -> Option<String> {
		match self {
			Self::string(string) => Some(string),
			_ => None
		}
	}
	pub fn is_file(&self) -> bool {
		match self {
			Self::file(_) => true,
			_ => false
		}
	}
	pub fn is_string(&self) -> bool {
		match self {
			Self::string(_) => true,
			_ => false
		}
	}
}

impl Default for res_keeper {
	fn default() -> Self { Self::None }
}

impl From<Option<File>> for res_keeper {
	fn from(item : Option<File>) -> Self
	{
		match item {
			Some(file) => res_keeper::file(file),
			None => res_keeper::None
		}
	}
}
impl From<Option<String>> for res_keeper {
	fn from(item : Option<String>) -> Self
	{
		match item {
			Some(string) => res_keeper::string(string),
			None => res_keeper::None
		}
	}
}
