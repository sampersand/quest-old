use crate::{Object, Shared, Result};
use std::path::{Path, PathBuf};
use std::{fs, io, sync::Mutex};
use super::parsable::{BUILTIN_PARSERS, ParsableStruct};
use crate::parse::{Parsable, ParseResult};

#[derive(Debug, Default, PartialEq)]
pub struct Parser {
	data: String,
	parsers: Shared<Vec<ParsableStruct>>,
	loc: Location,
	rollback: Shared<Vec<Object>>
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Location {
	source: Option<PathBuf>,
	line: usize,
	col: usize,
}

impl Parser {
	pub fn from_file(path: &Path) -> io::Result<Parser> {
		Ok(Parser {
			data: fs::read_to_string(path)?,
			loc: Location {
				source: Some(path.to_owned()),
				..Location::default()
			},
			parsers: BUILTIN_PARSERS.clone(),
			rollback: Shared::new(Vec::new())
		})
	}

	pub fn from_str(data: String) -> Parser {
		Parser {
			data,
			loc: Location::default(),
			parsers: BUILTIN_PARSERS.clone(),
			rollback: Shared::new(Vec::new())
		}
	}

	pub fn advance(&mut self, amount: usize) -> String {
		self.data.drain(..=amount).collect()
	}

	pub fn beginning(&self) -> String {
		if self.data.len() < 15 {
			self.data.clone()
		} else {
			format!("{}…", &self.data[..14])
		}
	}
}

impl AsRef<str> for Parser {
	fn as_ref(&self) -> &str {
		self.data.as_ref()
	}
}

// not using `Iterator` in case i want to modify it to return `Result` in the future
impl Parser {
	pub fn rollback(&self, obj: Object) {
		self.rollback.write().push(obj);
	}

	pub fn next_object(parser: &Shared<Parser>) -> Option<Result> {
		{
			let read = parser.read();
			let mut rollback = read.rollback.write();
			if let Some(obj) = rollback.pop() {
				trace!(target: "parse", "'Parsed' rolled-back obj={:?}", obj);
				return Some(Ok(obj));
			}
		}

		if parser.read().data.is_empty() {
			return None;
		}

		trace!(target: "parse", "Beginning parse. stream={:?}", parser.read().as_ref());

		let parsers = parser.read().parsers.clone();

		for parsablefn in parsers.read().iter() {
			match parsablefn.call(parser) {
				ParseResult::Restart => return Parser::next_object(parser),
				ParseResult::Ok(object) => return Some(object.handle(parser)),
				ParseResult::Err(err) => return Some(Err(err)),
				ParseResult::Eof => return None,
				ParseResult::None => { /* do nothing */ }
			}
		}

		Some(Err(crate::Error::NothingParsableFound(parser.clone())))
	}
}


impl_typed_object!(Shared<Parser>, variant Parser, new_parser, downcast_parser, is_parser);
impl_quest_conversion!("@parser" (as_parser_obj is_parser) (into_parser downcast_parser) -> Shared<Parser>);

impl_type! { for Shared<Parser>, downcast_fn=downcast_parser;
	// todo: stuff here
}