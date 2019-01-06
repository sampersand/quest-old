mod parser;

pub(crate) use self::parser::Parser;
use std::fmt::Debug;
use crate::{Shared, Environment, Result};
use std::{io, path::Path};

pub trait Parsable : Debug {
	fn try_parse(parser: Shared<Parser>) -> Option<Result>;
}


pub fn parse_file<P: AsRef<Path>>(path: P) -> io::Result<Result> {
	Parser::from_file(path.as_ref()).map(parse)
}

pub fn parse_str<T: Into<String>>(text: T) -> Result {
	parse(Parser::from_str(text.into()))
}

fn parse(parser: Parser) -> Result {
	Environment::execute(Environment::_new_default_with_stream(Shared::new(parser)))
}