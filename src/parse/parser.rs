use crate::{Object, Shared};
use std::path::{Path, PathBuf};
use std::{fs, io};


#[derive(Debug, Default)]
pub struct Parser {
	data: String,
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
			loc: Location { source: Some(path.to_owned()), ..Location::default() }
		})
	}

	pub fn from_str(data: String) -> Parser {
		Parser { data, loc: Location::default() }
	}
}

// not using `Iterator` in case i want to modify it to return `Result` in the future
impl Parser {
	pub fn next_object(parser: Shared<Parser>) -> Option<Object> {
		unimplemented!()
	}
}












