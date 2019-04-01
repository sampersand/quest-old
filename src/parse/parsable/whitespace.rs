use crate::object::Object;
use crate::parse::Parser;
use crate::parse::parsable::{Parsable, ParseResult};

#[derive(Debug, Clone, Copy)]
pub struct Whitespace;

impl Parsable for Whitespace {
	fn parse(parser: &Object<Parser>) -> ParseResult {
		let mut parser = parser.data().write().expect("write err in Whitespace::parse");
		let mut index = None;

		for (i, c) in parser.as_ref().chars().enumerate() {
			if c.is_whitespace() {
				index = Some(i);
			} else {
				break
			}
		}

		if let Some(index) = index {
			parser.advance(index);
			return ParseResult::Ok(Object::new_number(1f64).as_any());
			return ParseResult::Redo;
		} else {
			return ParseResult::None;
		}
	}
}
