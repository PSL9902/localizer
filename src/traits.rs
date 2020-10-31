use crate::Display;
use crate::HashMap;
pub trait Res : Sync + Send
{
	fn get_state(&self) -> &str;
	fn stringify(&mut self) -> Option<()>;//Result<(), LocError>;
	fn get_string(self) -> Option<String>;
	fn get_str(&self) -> Option<&str>;
	fn from_string(res : Option<String>) -> Self where Self: Sized;
	fn from_str(res : Option<&str>) -> Self where Self: Sized;
	fn set_string(&mut self, res : Option<String>);
	#[allow(patterns_in_fns_without_body)]
	fn user(mut self, f : &dyn Fn(&mut Self) -> ()) -> Self where Self: Sized;
}

pub trait FormArg {
	fn as_string(&self) -> String;
	//fn get_str<'a>(&'a self) -> &'a str;
}

pub trait ContFormArgs {
	fn get_val(&self, key: &str) -> Option<&dyn Display>;//Result<Box<dyn crate::Display>, LocError>;
	//Option<&'a dyn FormArg>;//Result<StdFormatterArgs, LocError>;
}
pub trait Form : Sync + Send {//<T:>
	fn format_1<'a>(&self, _ : &str, _ : &'a dyn ContFormArgs) -> Option<String>;
	//fn format_1<'a>(&self, _ : &str, _ : &'a HashMap<String, &'a dyn Display>) -> Option<String>;
	fn s_fmt(&self, a: &str) -> Option<String>;
	fn fmt(&self, a: &str, arg: &dyn ContFormArgs) -> Option<String>;//Result<String, LocError>;// where Self: Sized;
}

pub trait FormTo : Display {// + Clone
	fn as_str(&self) -> &str;
	
	fn default(&self) -> String;
	fn pretty(&self) -> String;
	fn clone(&self) -> Self;
}
impl FormTo for String {
	fn as_str(&self) -> &str {
		self.as_ref()
	}
	fn default(&self) -> String {
		format!("{}", self)
	}
	fn pretty(&self) -> String {
		format!("{:#}", self)
	}
	fn clone(&self) -> Self {
		self.to_string()
	}
}
impl FormTo for &str {
	fn as_str(&self) -> &str {
		self
	}
	fn default(&self) -> String {
		format!("{}", self)
	}
	fn pretty(&self) -> String {
		format!("{:#}", self)
	}
	fn clone(&self) -> Self {
		self
	}
}
/*
pub trait ToStr {
	fn to_str(&self) -> &str;
}
impl ToStr for String {
	fn to_str(&self) -> &str
	{
		self.as_ref()
	}
}
pub trait Container {
	fn get(&self, _: usize) -> Option<&dyn ToStr>;
	fn len(&self) -> usize;
	fn contains(&self, _: &dyn ToStr) -> bool;
}
impl Container for Vec<String> {
	#[allow(unconditional_recursion)]
	fn get(&self, index : usize) -> Option<&dyn ToStr> {
		self.get(index)
	}
	#[allow(unconditional_recursion)]
	fn len(&self) -> usize {
		self.len()
	}
	#[allow(unconditional_recursion)]
	fn contains(&self, key : &dyn ToStr) -> bool {
		self.contains(key)
	}
}
*/

/*
pub trait Ser {
	fn get(&self, string : &str, key : &str, current_lang : &Option<String>) -> Option<String>;
	fn get1(&self, string : &str, key : &str) -> Vec<String>;
}
*/

/*
pub enum LocError {
	
}

*/
