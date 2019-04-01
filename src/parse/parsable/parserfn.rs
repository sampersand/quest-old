use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Formatter};
use crate::object::Object;
use crate::parse::{Parser, Parsable, ParseResult};

#[derive(Clone, Copy)]
pub struct ParsableFn(&'static str, fn(&Object<Parser>) -> ParseResult);

impl ParsableFn {
	pub fn new<T: Parsable>() -> ParsableFn {
		ParsableFn(type_name::get::<T>(), T::parse)
	}

	pub fn parse(&self, parser: &Object<Parser>) -> ParseResult {
		(self.1)(parser)
	}
}

impl Debug for ParsableFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "ParsableFn({}, {:p})", self.0, self.1 as *const ())
	}
}

impl Eq for ParsableFn {}
impl PartialEq for ParsableFn {
	fn eq(&self, other: &ParsableFn) -> bool {
		(self.1 as usize) == (other.1 as usize)
	}
}

impl Hash for ParsableFn {
	fn hash<H: Hasher>(&self, h: &mut H) {
		(self.1 as usize).hash(h)
	}
}