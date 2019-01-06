mod err;
mod parser;

pub use self::err::{Error, Result};

use crate::{Shared, Environment};
pub(crate) use self::parser::Parser;
use std::{io, path::Path};

pub fn parse_file<P: AsRef<Path>>(path: P) -> io::Result<crate::Result> {
	Parser::from_file(path.as_ref()).map(parse)
}

pub fn parse_str<T: Into<String>>(text: T) -> crate::Result {
	parse(Parser::from_str(text.into()))
}

fn parse(parser: Parser) -> crate::Result {
	Environment::execute(Environment::_new_default_with_stream(Shared::new(parser)))
}