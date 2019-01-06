use crate::Shared;
use crate::parse::{Parsable, ParseResult, Parser};

pub(super) struct Whitespace; 

impl Parsable for Whitespace {
	const NAME: &'static str = "Whitespace";
	fn try_parse(parser: &Shared<Parser>) -> ParseResult {
		let mut idx = 0;
		
		for (i, c) in parser.read().as_ref().chars().enumerate() {
			if c.is_whitespace() {
				idx = i;
			} else {
				break
			}
		}

		if idx == 0 {
			trace!(target: "parser", "No whitespace found. stream={:?}", parser.read().beginning());
			ParseResult::None
		} else {
			let whitespace = parser.write().advance(idx); // ignore whatever whitespace we had
			debug_assert!(whitespace.chars().all(char::is_whitespace), "invalid whitespace parsed: {:?}", whitespace);
			debug!(target: "parser", "Whitespace parsed. chars={:?}", whitespace);
			ParseResult::Restart
		}
	}
}