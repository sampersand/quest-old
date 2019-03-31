mod whitespace;
mod parserfn;

pub use self::parserfn::ParserFn;
pub type Parsers = Vec<ParserFn>;

use crate::parse::Translator;
use crate::object::{Object, AnyObject};
use crate::error::Error;

pub enum ParseResult {
	Ok(AnyObject),
	Err(Error),
	None,
	Redo,
	Eof,
}

pub trait Parser : std::fmt::Debug + Send + Sync {
	fn parse(parser: &Object<Translator>) -> ParseResult;
}

pub fn _default_parsers() -> Parsers {
	vec![
		ParserFn::new::<whitespace::Whitespace>()
	]
}