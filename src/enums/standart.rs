#![cfg(not(feature = "no_std"))]
use std::fs::File;
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum res_keeper
{
	file(File),
	string(String),
	None,
}
impl res_keeper {
	pub fn new_file(res : Option<File>) -> Self {
		match res {
			Some(file) => Self::file(file),
			None => Self::None,
		}
	}
	pub fn new_file_from_path(res : Option<&std::path::Path>) -> Self {
		use crate::constants::STD_PATH;
		use std::fs::OpenOptions;
		let file = OpenOptions::new()
		.read(true)
		.open(
			match res {
				Some(path) => path.to_str().unwrap_or(STD_PATH),
				None => STD_PATH,
			}
		);
		#[cfg(debug)]
		println!("{:?}", file);
		Self::new_file(file.ok())
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
	pub fn set_file_from_path(&mut self, path : Option<&std::path::Path>) {
		use crate::constants::STD_PATH;
		use std::fs::OpenOptions;
		let file = OpenOptions::new().read(true)
		.open( match path { Some(path) => path.to_str().unwrap_or(STD_PATH), None => STD_PATH, } );
		#[cfg(debug)]
		println!("{:?}", file);
		self.set_file(file.ok());
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

impl From<Option<res_keeper>> for res_keeper {
	fn from(item : Option<res_keeper>) -> Self
	{
		match item {
			Some(res) => res,
			None => res_keeper::None
		}
	}
}

impl crate::Res for res_keeper {
	fn get_state(&self) -> &str {match self{Self::file(_) => "file", Self::string(_) => "string", Self::None => "None"}}
	
	fn stringify(&mut self) -> Option<()> {//Result<(), dyn LocError + 'static > {//
		use std::io::{BufReader, Read};
		match self {
			Self::string(_) => Some(()),
			Self::file(file) => {
					let mut buf_reader = BufReader::new(file);
					let mut string = String::new();
					buf_reader.read_to_string(&mut string).ok()?;//.unwrap();//
					*self = Self::string(string);
					Some(())
				},
			Self::None => None,
		}
	}
	fn get_string(self) -> Option<String> {
		use std::io::{BufReader, Read};
		match self {
			Self::string(string) => Some(string),
			Self::file(file) => {
					let mut buf_reader = BufReader::new(file);
					let mut string = String::new();
					buf_reader.read_to_string(&mut string).ok()?;//.unwrap();//
					Some(string)
				},
			Self::None => None,
		}
	}
	fn from_string(res : Option<String>) -> Self {
		Self::new_string(res)
	}
	fn from_str(res : Option<&str>) -> Self {
		Self::new_string(res.map(|res| res.to_string()))
	}
	fn set_string(&mut self, res : Option<String>) {
		self.set_string(res);
	}
	
	fn user(mut self, f : &dyn Fn(&mut Self) -> ()) -> Self {
		f(&mut self);
		self
	}
	
	fn get_str(&self) -> Option<&str> {
		match self {
			Self::string(ref string) => Some(string),
			_ => None,
		}
	}
}
