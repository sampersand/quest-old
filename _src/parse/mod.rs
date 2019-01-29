mod parser;
mod parsable;
mod result;

pub use self::parser::Parser;
pub use self::parsable::{Parsable, ParseFromStr, ParseOk};
pub use self::result::Result;

use std::{io, path::Path};
use crate::{Environment, Shared, Object};

pub fn parse_file<P: AsRef<Path>>(path: P, parent: Option<Shared<Environment>>) -> crate::Result<Object> {
	parse(Parser::from_file(path.as_ref()).map_err(crate::err::Error::IoError)?, parent)
}

pub fn parse_str<T: Into<String>>(text: T, parent: Option<Shared<Environment>>) -> crate::Result<Object> {
	parse(Parser::from_str(text.into()), parent)
}

fn parse(parser: Parser, parent: Option<Shared<Environment>>) -> crate::Result<Object> {
	let env = Environment::execute(
		Environment::_new_default_with_stream_using_parent_stack(Shared::new(parser), parent)
	)?;
	let res = env.read().stack.write().pop().ok_or_else(|| crate::err::Error::NothingToReturn);
	drop(env);
	res
}