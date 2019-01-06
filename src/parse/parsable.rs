use crate::parse::Parser;
use crate::{Shared, Result};
use std::fmt::{self, Debug, Formatter};
use lazy_static::lazy_static;
 
pub struct ParsableStruct(&'static str, fn(&Shared<Parser>) -> Option<Result>);

pub trait Parsable {
	const NAME: &'static str;
	fn try_parse(parser: &Shared<Parser>) -> Option<Result>;
}

lazy_static! {
	pub static ref BUILTIN_PARSERS: Shared<Vec<ParsableStruct>> = Shared::new(vec!{
		
	});
}

impl Debug for ParsableStruct {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "ParsableStruct({:?}, {:p})", self.0, self.1 as *const ())
		} else {
			write!(f, "ParsableStruct({:?})", self.0)
		}
	}
}

impl ParsableStruct {
	pub fn call(&self, parser: &Shared<Parser>) -> Option<Result> {
		(self.1)(parser)
	}
}