use crate::{Shared, Object};
use crate::parse::{self, Parsable, Parser};

pub(super) struct Whitespace; 

named!(Whitespace);

impl Parsable for Whitespace {
	fn try_parse(parser: &Shared<Parser>) -> parse::Result<Object> {
		let mut idx = None;
		
		for (i, c) in parser.read().as_ref().chars().enumerate() {
			if c.is_whitespace() {
				idx = Some(i);
			} else {
				break
			}
		}

		if let Some(index) = idx {
			let whitespace = parser.write().advance(1 + index); // ignore whatever whitespace we had
			debug_assert!(whitespace.chars().all(char::is_whitespace), "invalid whitespace parsed: {:?}", whitespace);
			debug!(target: "parser", "Whitespace parsed. chars={:?}", whitespace);
			parse::Result::Restart
		} else {
			trace!(target: "parser", "No whitespace found. stream={:?}", parser.read().beginning());
			parse::Result::None
		}
	}
}