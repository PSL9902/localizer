use crate::ContFormArgs;
use crate::{HashMap, Display};

pub struct CFArgs<'a> {
	item : HashMap<String, &'a dyn Display>
	//item : HashMap<String, Box<dyn Display + 'static>>
	/*
	item : Vec<StdFormatterArgs>,
	ptr  : usize,*/
}

/*
impl<'a> Iterator for CFArgs {
	type Item = &'a crate::enums::StdFormatterArgs;
	fn next(&'a mut self) -> Option<Self::Item> {
		let res = self.item.get(self.ptr)?;
		self.ptr += 1;
		Some(res)
	}
}*/
impl<'a> CFArgs<'a> {
	pub fn new(item : &HashMap<String, &dyn Display>) -> Self {
		Self{ item: item.iter().map(|(x, y)| (x.to_string(), *y)).collect()}//item.into()}//
	}
	/*
	pub fn new(item : Vec<StdFormatterArgs>) -> Self {
		Self{item, ptr : 0,}
	}
	pub fn reset(&mut self) {
		self.ptr = 0;
	}
	pub fn next<'a>(&'a mut self) -> Option<&'a crate::enums::StdFormatterArgs> {
		let res = self.item.get(self.ptr)?;
		self.ptr += 1;
		Some(res)
	}*/
}

impl ContFormArgs for CFArgs<'_> {
	fn get_val<'a>(&'a self, key: &str) -> Option<&'a dyn Display>//Result<StdFormatterArgs, LocError>
	{
		self.item.get(key).map(|x| *x)//.map(|x| Box::new(x.as_ref()))
	}
}
