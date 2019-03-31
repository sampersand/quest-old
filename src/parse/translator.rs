use crate::object::{Object, AnyObject};
use crate::error::Result;
use crate::parse::Parsers;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Translator {
	loc: Location,
	data: String,
	parsers: Parsers
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location {
	filename: Option<PathBuf>,
	lineno: usize,
	column: usize
}

impl Translator {
	pub fn from_str(data: &str, parsers: Option<Parsers>) -> Object<Translator> {
		let parsers = parsers.unwrap_or_else(super::parser::_default_parsers);

		Object::new(Translator {
			loc: Location { filename: None, lineno: 0, column: 0 },
			data: data.to_string(),
			parsers
		})
	}
}

impl Object<Translator> {
	pub fn parse(&self) -> Result<AnyObject> {
		unimplemented!()
	}
}

impl_type! { for Translator; }


