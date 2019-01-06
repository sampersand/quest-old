use crate::{Shared, IntoObject};
use crate::parse::{Parsable, ParseResult, Parser};

pub use crate::object::typed::Number;

impl Parsable for Number {
	const NAME: &'static str = "Number";
	fn try_parse(parser: &Shared<Parser>) -> ParseResult {
		let number = Number::from_str(parser.read().as_ref());

		if let Some((number, index)) = number {
			let mut parser = parser.write();
			let res = parser.advance(index);
			debug_assert_eq!(number, Number::from_str(&res).unwrap().0);
			debug!(target: "parser", "Number parsed. chars={:?}", res);
			ParseResult::Ok(number.into_object())
		} else {
			trace!(target: "parser", "No number found. stream={:?}", parser.read().beginning());
			ParseResult::None
		}
	}
}