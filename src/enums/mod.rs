#![allow(dead_code)]

#[cfg(not(feature = "no_std"))]
pub mod standart;
#[cfg(not(feature = "no_std"))]
pub use standart::*;

#[cfg(feature = "format")]
pub mod form;
#[cfg(feature = "format")]
pub use form::*;

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
	pub fn get(&self, string : &str, key : &str, current_lang : &Option<String>) -> Option<String> {
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
						
						
						let d = parse(&string).unwrap();//.ok()?;
						//println!("{:?}", d);
						if !d.is_object()
						{
							#[cfg(debug)]
							dbg!(d);
							#[cfg(not(debug))]
							unimplemented!();
						}
						//println!("{:?}", d);
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
						//println!("{:?}", table);
						
						let langs = as_table(table.get("langs")?)?;
						//println!("{:?}", langs);
						
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
	pub fn get1(&self, string : &str, key : &str) -> Vec<String> {
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
