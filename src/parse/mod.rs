mod parser;
mod parsable;

pub use self::parser::Parser;
pub use self::parsable::{Parsable, ParseResult};

use crate::{Result, Error};
use std::{io, path::Path};


pub fn parse_file<P: AsRef<Path>>(path: P) -> Result {
	parse(Parser::from_file(path.as_ref()).map_err(Error::IoError)?)
}

pub fn parse_str<T: Into<String>>(text: T) -> Result {
	parse(Parser::from_str(text.into()))
}

fn parse(parser: Parser) -> Result {
	use crate::{Environment, Shared};
	Environment::execute(Environment::_new_default_with_stream(Shared::new(parser)))
}