use std::fmt::{self, Debug, Formatter};
use crate::{Shared, Result};
use crate::parse::{Parser, Parsable, ParseResult};

#[derive(Clone, Copy)]
pub struct ParsableStruct(&'static str, fn(&Shared<Parser>) -> ParseResult);


impl Eq for ParsableStruct {}
impl PartialEq for ParsableStruct {
	fn eq(&self, other: &ParsableStruct) -> bool {
		(self.0 == other.0) && (self.1 as usize == other.1 as usize)
	}
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
	pub fn new<T: Parsable>() -> ParsableStruct {
		ParsableStruct(T::NAME, T::try_parse)
	}

	pub fn call(&self, parser: &Shared<Parser>) -> ParseResult {
		(self.1)(parser)
	}
}