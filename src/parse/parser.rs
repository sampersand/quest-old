use crate::{Object, Shared, Result};
use std::path::{Path, PathBuf};
use std::{fs, io};
use super::parsable::{Parsable, ParsableStruct, BUILTIN_PARSERS};

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
}

// not using `Iterator` in case i want to modify it to return `Result` in the future
impl Parser {
	pub fn next_object(parser: Shared<Parser>) -> ::std::result::Result<Option<Object>, crate::Error> {
		let parsers = parser.read().parsers.clone();
		for parsablefn in parsers.read().iter() {
			if let Some(obj) = parsablefn.call(&parser).transpose()? {
				return Ok(Some(obj))
			}
		}

		Err(crate::Error::NothingParsableFound(parser))
	}
}












