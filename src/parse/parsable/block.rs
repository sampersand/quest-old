use crate::{Shared, IntoObject, Object};
use crate::parse::{self, Parsable, Parser};

pub use crate::object::typed::Block;

impl Parsable for Block {
	const NAME: &'static str = "Block";
	fn try_parse(parser: &Shared<Parser>) -> parse::Result<Object> {
		let number = Block::parse(parser.read().as_ref());

		if let Some((number, index)) = number {
			let mut parser = parser.write();
			let res = parser.advance(index-1);
			debug_assert_eq!(number, Block::parse(&res).unwrap().0);
			debug!(target: "parser", "Block parsed. chars={:?}", res);
			parse::Result::Ok(number.into_object())
		} else {
			trace!(target: "parser", "No block found. stream={:?}", parser.read().beginning());
			parse::Result::None
		}
	}
}