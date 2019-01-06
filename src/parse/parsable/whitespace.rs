use crate::Shared;
use crate::parse::{Parsable, ParseResult, Parser};

pub(super) struct Whitespace; 

impl Parsable for Whitespace {
	const NAME: &'static str = "Whitespace";
	fn try_parse(parser: &Shared<Parser>) -> ParseResult {
		let mut idx = None;
		
		for (i, c) in parser.read().as_ref().chars().enumerate() {
			if c.is_whitespace() {
				idx = Some(i);
			} else {
				break
			}
		}

		if let Some(index) = idx {
			let whitespace = parser.write().advance(index); // ignore whatever whitespace we had
			debug_assert!(whitespace.chars().all(char::is_whitespace), "invalid whitespace parsed: {:?}", whitespace);
			debug!(target: "parser", "Whitespace parsed. chars={:?}", whitespace);
			ParseResult::Restart
		} else {
			trace!(target: "parser", "No whitespace found. stream={:?}", parser.read().beginning());
			ParseResult::None
		}
	}
}