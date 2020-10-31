use crate::{Form, FormTo, FormArg, ContFormArgs, HashMap};
use crate::enums::{StdFormatterArgs};//pub 
//use super::Localizer;
use crate::Display;

mod container;
pub use container::CFArgs;

//#[derive(Debug, Clone, PartialEq)]
pub struct StdFormatter {//<T:FormTo>
	res : Option<Box<dyn ContFormArgs + Send + Sync>>//Option<CFArgs>,//Option<HashMap<String, T>>,
}
//impl<T:FormTo> StdFormatter<T> {//
impl StdFormatter {
	pub fn new() -> Self{Self{res : None}}
	fn compile(mut self, arr : &[/*T*/&dyn Display]) -> Self {// -> HashMap<String, T> {
		let mut ind = 0;
		self.res = Some(Box::new(CFArgs::new(&arr.iter().map(|x| (format!("ind_{}", (ind, ind+=1).0), x.clone())).collect())));
		self
	}
	/*fn compile(mut self, arr : &[/*T*/impl FormTo]) -> Self {// -> HashMap<String, T> {
		let mut ind = 0;
		self.res = Some(Box::new(CFArgs::new(&arr.iter().map(|x| (format!("ind_{}", (ind, ind+=1).0), x.clone())).collect())));
		self
	}*/
	pub fn add_j(mut self, a : impl ContFormArgs + 'static + Sync) -> Self {
		self.res=Some(Box::new(a));
		self
	}
	//fn format<T:FormTo>(keys : &str, args : &HashMap<String, T>) -> Option<String> {
	fn format(&self, keys : &str) -> Option<String> {
		let args = self.res.as_ref()?;
		let mut res = keys.to_string();
		for (ind, range) in crate::finds(&mut res).into_iter().enumerate() {
			res.replace_range(range.0..range.1, args.get_val(&format!("ind_{}", ind))?.as_str());
		}
		Some(res)
	}
	pub fn build(a : Vec<(&str, &str)>) -> HashMap<String, StdFormatterArgs> {
		a.into_iter().map(|x|
		( x.0.to_string(),
		  StdFormatterArgs::Replmnt(Member::new(x.1.to_string())),
		)).collect::<HashMap<_, _>>()
	}
	pub fn build_1<'a >(a : &[&'a dyn crate::Display]) -> dyn ContFormArgs + 'static {//HashMap<String, &'a dyn crate::Display>
		a.iter().enumerate().map(|(ind, x)| (format!("ind_{}", ind), *x)).collect()
	}
}

//impl<T:FormTo + Send + Sync> Form for StdFormatter<T> {
impl Form for StdFormatter {
	fn s_fmt(&self, a: &str) -> Option<String> {
		self.format(a)
	}
	fn format_1(&self, keys : &str, args : &dyn ContFormArgs) -> Option<String> {
		let mut res = keys.to_string();
		let r = crate::finds_all(&mut res);
		for item in r.into_iter() {
			res.replace_range(item.0 .. item.1, format!("{}", args.get_val(&item.2)?).as_str());
		}
		Some(res)
	}
	fn fmt(&self, a: &str, _arg: &dyn ContFormArgs) -> Option<String> {
		self.format(a)
		/*
		let args = self.res.as_ref()?;
		let mut ind = 0;
		
		for (ind, ind1) in crate::finds(&mut res).into_iter() {
			let mut d = &res[ind+1 .. ind1-1];
			if let Some(ind1) = d.find(":") {
				d = &res[ind+1 .. ind1];
			}
			let key = arg.next()?.as_string();
			res.replace_range(ind .. ind1, args.get(&format!("ind_{}", d))?.as_str());
		}*/
		/*
		while ind < a.len() {
			let y = finds(a)?;
			res.push_str(&a[ind..y.0]);
			res.push_str(&arg.get_val(&a[(y.0 + 1)..y.1])?.as_string());////get
			ind = y.1 + 1;
		}*/
		//Some(res)
	}
	//fn add
	//fn build()
	//fn get(&str) -> Option<String>
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
	Center,
	Right,
	Left,
	None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
	pub item : String,
	pub padding: (usize, Side),
}

impl Member {
	pub fn build<T:FormTo>(self, a:&HashMap<String, T>) -> Option<String> {
		Some(a.get(&self.item)?.default())
	}
	pub fn new(item : String) -> Self {
		Self{item, padding: (0, Side::None)}
	}
}

