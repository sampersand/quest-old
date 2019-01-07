use crate::{Object, Shared, Result};
use std::path::{Path, PathBuf};
use std::{fs, io};
use super::parsable::{BUILTIN_PARSERS, ParsableStruct};
use crate::parse::{Parsable, ParseResult};

#[derive(Debug, Default)]
pub struct Parser {
	data: String,
	parsers: Shared<Vec<ParsableStruct>>,
	loc: Location,
}

#[derive(Debug, Default)]
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
			parsers: BUILTIN_PARSERS.clone()
		})
	}

	pub fn from_str(data: String) -> Parser {
		Parser {
			data,
			loc: Location::default(),
			parsers: BUILTIN_PARSERS.clone()
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
	pub fn next_object(parser: &Shared<Parser>) -> Option<Result> {
		trace!(target: "parse", "Beginning parse. stream={:?}", parser.read().as_ref());
		let parsers = parser.read().parsers.clone();
		println!("{:?}", crate::env::Environment::current());
		for parsablefn in parsers.read().iter() {
			match parsablefn.call(parser) {
				ParseResult::Restart => return Parser::next_object(parser),
				ParseResult::Ok(object) => return Some(Ok(object)),
				ParseResult::Err(err) => return Some(Err(err)),
				ParseResult::Eof => return None,
				ParseResult::None => { /* do nothing */ }
			}
		}

		Some(Err(crate::Error::NothingParsableFound(parser.clone())))
	}
}












