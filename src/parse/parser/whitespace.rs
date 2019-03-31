use crate::object::Object;
use crate::parse::Translator;
use crate::parse::parser::{Parser, ParseResult};

#[derive(Debug, Clone, Copy)]
pub struct Whitespace;

impl Parser for Whitespace {
	fn parse(parser: &Object<Translator>) -> ParseResult {
		unimplemented!()
	}
}
