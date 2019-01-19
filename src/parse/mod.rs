mod parser;
mod parsable;
mod result;

pub use self::parser::Parser;
pub use self::parsable::Parsable;
pub use self::result::Result;

use std::{io, path::Path};


pub fn parse_file<P: AsRef<Path>>(path: P) -> crate::Result<crate::Object> {
	parse(Parser::from_file(path.as_ref()).map_err(crate::err::Error::IoError)?)
}

pub fn parse_str<T: Into<String>>(text: T) -> crate::Result<crate::Object> {
	parse(Parser::from_str(text.into()))
}



fn parse(parser: Parser) -> crate::Result<crate::Object> {
	use crate::{Environment, Shared, Object};
	let env = Environment::execute(
		Environment::_new_default_with_stream(Shared::new(parser))
	)?;
	println!("{:#?}", env);
	let res = env.read().stack.write().pop().ok_or_else(|| crate::err::Error::NothingToReturn);
	drop(env);
	res
}