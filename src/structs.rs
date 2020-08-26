#![allow(dead_code)]
use std::{fs::OpenOptions, path::Path, io::{BufReader, Read}};

#[path="enums.rs"]
mod enums;
pub use enums::*;

#[path="constants.rs"]
mod constants;
pub use constants::*;


#[derive(Debug)]
pub struct Localizer
{
	pub res : res_keeper,
	pub current_lang : Option<String>,
	pub ser_form : serialize_form,
}
impl Localizer {
	pub const fn new(res : res_keeper, current_lang : Option<String>, ser_form : serialize_form) -> Self {
		Self { res, current_lang, ser_form }
	}
	pub const fn create() -> Self {
		Self { res : res_keeper::None, current_lang : None, ser_form : serialize_form::toml }
	}
	
	pub fn file(mut self, path : Option<&Path>) -> Self {
		let file = OpenOptions::new()
		.read(true)
		.open(
			match path {
				Some(path) => path.to_str().unwrap_or(STD_PATH),
				None => STD_PATH,
			}
		);
		#[cfg(debug)]
		println!("{:?}", file);
		//self.res = res_keeper::new_file(file.ok());
		//self.res = file.ok().into();
		self.res.set_file(file.ok());
		self
	}
	pub fn string(mut self, string : Option<String>) -> Self {
		//self.res = res_keeper::new_string(string);
		//self.res = string.into();
		self.res.set_string(string);
		self
	}
	pub fn current_lang(mut self, lang : Option<String>) -> Self {
		self.current_lang = lang;
		self
	}
	pub fn ser_form(mut self, form : Option<serialize_form>) -> Self {
		self.ser_form = form.unwrap_or(serialize_form::toml);
		self
	}
	pub fn user(mut self, f : &dyn Fn(&mut Self) -> ()) -> Self {//// RENAME
		f(&mut self);
		self
	}
	
	
	pub fn set_file(&mut self, path : Option<&Path>) -> &mut Self {
		let file = OpenOptions::new().read(true)
		.open( match path { Some(path) => path.to_str().unwrap_or(STD_PATH), None => STD_PATH, } );
		#[cfg(debug)]
		println!("{:?}", file);
		self.res.set_file(file.ok());
		self
	}
	pub fn set_string(&mut self, string : Option<String>) -> &mut Self {
		self.res.set_string(string);
		self
	}
	pub fn set_current_lang(&mut self, lang : Option<String>) -> &mut Self {
		self.current_lang = lang;
		self
	}
	pub fn set_ser_form(&mut self, form : Option<serialize_form>) -> &mut Self {
		self.ser_form = form.unwrap_or(serialize_form::toml);
		self
	}
	
	
	pub fn into_string(&mut self) -> Option<()> {
		Some(match &self.res {
			res_keeper::file(file) => {
					let mut buf_reader = BufReader::new(file);
					let mut string = String::new();
					buf_reader.read_to_string(&mut string).ok()?;//.unwrap();//
					self.set_string(Some(string));
				}
			_ => (),
		})
	}
	
	pub fn get(&self, key : &str) -> Option<String> {
		#[cfg(debug)]
		println!("{:?}", self.res);
		match &self.res
		{
			res_keeper::file(file) => {
					let mut buf_reader = BufReader::new(file);
					let mut string = String::new();
					buf_reader.read_to_string(&mut string).ok()?;//.unwrap();//
					
					self.ser_form.get(&string, key, &self.current_lang)
				}
			res_keeper::string(ref string) => { self.ser_form.get(string, key, &self.current_lang) }
			res_keeper::None => None,
		}
	}
	pub fn get1(&self, key : &str) -> Vec<String> {
		#[cfg(debug)]
		println!("{:?}", self.res);
		match &self.res
		{
			res_keeper::file(file) => {
					let mut buf_reader = BufReader::new(file);
					let mut string = String::new();
					match buf_reader.read_to_string(&mut string) {
						Ok(_) => (),
						_ => return Vec::new(),
					};
					self.ser_form.get1(&string, key)
				}
			res_keeper::string(ref string) => { self.ser_form.get1(string, key) }
			res_keeper::None => Vec::new(),
		}
	}
}

impl Default for Localizer {
	fn default() -> Self { Self::create() }
}
