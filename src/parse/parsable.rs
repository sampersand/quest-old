mod whitespace;
mod parserfn;

pub use self::parserfn::ParsableFn;
pub type Parsables = Vec<ParsableFn>;

use crate::parse::Parser;
use crate::object::{Object, AnyObject};
use crate::error::Error;

pub enum ParseResult {
	Ok(AnyObject),
	Err(Error),
	None,
	Redo,
	Eof,
}

pub trait Parsable : std::fmt::Debug + Send + Sync {
	fn parse(parser: &Object<Parser>) -> ParseResult;
}

pub fn _default_parsers() -> Parsables {
	vec![
		ParsableFn::new::<whitespace::Whitespace>()
	]
}