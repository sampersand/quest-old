use crate::{Shared, IntoObject};
use crate::parse::{Parsable, ParseResult, Parser};

pub use crate::object::typed::Block;

impl Parsable for Block {
	const NAME: &'static str = "Block";
	fn try_parse(parser: &Shared<Parser>) -> ParseResult {
		let number = Block::parse(parser.read().as_ref());

		if let Some((number, index)) = number {
			let mut parser = parser.write();
			let res = parser.advance(index-1);
			debug_assert_eq!(number, Block::parse(&res).unwrap().0);
			debug!(target: "parser", "Block parsed. chars={:?}", res);
			ParseResult::Ok(number.into_object())
		} else {
			trace!(target: "parser", "No block found. stream={:?}", parser.read().beginning());
			ParseResult::None
		}
	}
}