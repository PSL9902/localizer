#![allow(dead_code)]

use crate::{Res, Form, ContFormArgs};//
pub use crate::constants::*;
use crate::enums::{serialize_form};//pub 
use crate::HashMap;


//#[derive(Debug, Clone, PartialEq)]
pub struct Localizer
{
	pub res : Option<Box<dyn Res>>,
	pub current_lang : Option<String>,
	pub ser_form : serialize_form,
	#[cfg(feature = "format")]
	pub formatter : Option<Box<dyn Form>>,
}
impl Localizer {
	#[cfg(feature = "format")]
	pub /*const*/ fn new(res : impl Res + 'static/*Option<impl Res + 'static>*/, current_lang : Option<String>, ser_form : serialize_form, formatter : Option<Box<dyn Form>>) -> Self {
		Self { res : Some(Box::new(res)), current_lang, ser_form, formatter }
	}
	#[cfg(not(feature = "format"))]
	pub /*const*/ fn new(res : impl Res + 'static/*Option<impl Res + 'static>*/, current_lang : Option<String>, ser_form : serialize_form) -> Self {
		Self { res : Some(Box::new(res)), current_lang, ser_form }
	}
	
	
	#[cfg(feature = "format")]
	pub /*const*/ fn create() -> Self {
		Self { res : None, current_lang : None, ser_form : serialize_form::toml, formatter : None }
	}
	#[cfg(not(feature = "format"))]
	pub /*const*/ fn create() -> Self {
		Self { res : None, current_lang : None, ser_form : serialize_form::toml}
	}
	
	pub fn res(mut self, res : impl Res + 'static) -> Self {
	//pub fn res(mut self, res : Option<impl Res + 'static>) -> Self {
	//	self.res = res.map(|res| Box::new(res));
		self.res = Some(Box::new(res));
		self
	}
	pub fn string(mut self, string : Option<String>) -> Self {
		self.res.as_mut().map(|res| res.set_string(string));
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
	
	#[cfg(feature = "format")]
	pub fn formatter(mut self, form : impl Form + 'static) -> Self {
		self.formatter = Some(Box::new(form));
		self
	}
	pub fn user(mut self, f : &dyn Fn(&mut Self) -> ()) -> Self {
		f(&mut self);
		self
	}
	
	pub fn set_res(&mut self, res : Option<impl Res + 'static>) -> &mut Self {
		self.res = match res {
			Some(res) => Some(Box::new(res)),
			_ => None,
		};
		self
	}
	
	pub fn set_string(&mut self, string : Option<String>) -> &mut Self {
		self.res.as_mut().map(|res| res.set_string(string));
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
	#[cfg(feature = "format")]
	pub fn set_formatter(&mut self, form : Option<Box<dyn Form>>) -> &mut Self {
		self.formatter = form;
		self
	}
	
	pub fn into_string(&mut self) -> Option<()> {//Result<(), dyn LocError + 'static> {
		/*match self.res {
			Some(ref mut x) => x.as_mut().stringify(),
			_ => None,
		}*/
		self.res.as_mut().map(|res| res.stringify()).flatten()
	}
	
	pub fn get(&self, key : &str) -> Option<String> {
		#[cfg(debug)]
		println!("{:?}", self.res);
		self.ser_form.get(self.res.as_ref()?.get_str()?, key, &self.current_lang)
	}
	#[cfg(feature = "format")]
	pub fn get_f(&self, key : &str) -> Option<String> {
		let res = self.get(key);
		match self.formatter {
			Some(ref fo) => fo.s_fmt(res?.as_ref()),
			None => res,
		}
	}
	#[cfg(feature = "format")]
	pub fn get_f1(&self, key : &str, args : &dyn ContFormArgs) -> Option<String> {
		let res = self.get(key);
		match self.formatter {
			Some(ref fo) => fo.format_1(res?.as_ref(), args),
			None => res,
		}
	}
	/*
	pub fn get_f_1<T:crate::FormTo + Send + Sync>(&self, key : &str, args : &HashMap<String, T>) -> Option<String> {
		let res = self.get(key);
		match self.formatter {
			Some(ref fo) => fo.format_1(res?.as_ref(), args),
			None => res,
		}
	}*/
	pub fn get1(&self, key : &str) -> Vec<String> {
		#[cfg(debug)]
		println!("{:?}", self.res);
		self.res.as_ref()
		.map(|res| res.get_str())
		.flatten()
		.map(|string| self.ser_form.get1(string, key))
		.unwrap_or(Vec::new())
		/*match self.res.as_ref().map(|res| res.get_str()).flatten() {
			Some(string) => self.ser_form.get1(string, key),
			_ => Vec::new(),
		}*/
	}
}

impl Default for Localizer {
	fn default() -> Self { Self::create() }
}
