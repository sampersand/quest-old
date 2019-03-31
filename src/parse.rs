mod translator;
mod parser;
pub mod error;

use self::parser::Parsers;
use self::translator::Translator;
pub use self::error::Error;

use std::path::Path;
use crate::error::Result;
use crate::object::AnyObject;

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<AnyObject> {
	// parser::from_path(path).parse()
	unimplemented!()
}

pub fn parse_str(inp: &str, parsers: Option<Parsers>) -> Result<AnyObject> {
	Translator::from_str(inp, parsers).parse()
}