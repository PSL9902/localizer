
#[derive(Debug, Clone, PartialEq)]
pub enum StdFormatterArgs {
	Fill(crate::Member, usize),
	Replmnt(crate::Member),
}
impl StdFormatterArgs {
	fn build(&self) -> String {
		match self {
			Self::Fill(a, count) => {
					let mut res = String::new();
					for _ in 0..*count {res.push_str(&a.item);}
					res
				},
			Self::Replmnt(a) => a.item.to_string(),
			#[allow(unreachable_patterns)]
			_ => unimplemented!(),
		}
	}
}
/*
impl AsRef for StdFormatterArgs {
	fn as_ref(&self) -> &str {
		self.build()
	}
}
*/

impl crate::FormArg for StdFormatterArgs {
	fn as_string(&self) -> String {
		self.build()
	}
	/*fn build(a: StdFormatterArgs) -> Box<dyn crate::FormArg> {
		Box::new(a)
	}*/
}
