use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Formatter};
use crate::object::Object;
use crate::parse::{Translator, parser::{Parser, ParseResult}};

#[derive(Clone, Copy)]
pub struct ParserFn(&'static str, fn(&Object<Translator>) -> ParseResult);

impl ParserFn {
	pub fn new<T: Parser>() -> ParserFn {
		ParserFn(type_name::get::<T>(), T::parse)
	}
}

impl Debug for ParserFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "ParserFn({}, {:p})", self.0, self.1 as *const ())
	}
}

impl Eq for ParserFn {}
impl PartialEq for ParserFn {
	fn eq(&self, other: &ParserFn) -> bool {
		(self.1 as usize) == (other.1 as usize)
	}
}

impl Hash for ParserFn {
	fn hash<H: Hasher>(&self, h: &mut H) {
		(self.1 as usize).hash(h)
	}
}