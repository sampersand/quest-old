mod parser;
mod parsable;

pub(crate) use self::parser::Parser;

use crate::{Shared, Environment, Result};
use std::{io, path::Path};
pub use self::parsable::Parsable;


pub fn parse_file<P: AsRef<Path>>(path: P) -> io::Result<Result> {
	Parser::from_file(path.as_ref()).map(parse)
}

pub fn parse_str<T: Into<String>>(text: T) -> Result {
	parse(Parser::from_str(text.into()))
}

fn parse(parser: Parser) -> Result {
	Environment::execute(Environment::_new_default_with_stream(Shared::new(parser)))
}