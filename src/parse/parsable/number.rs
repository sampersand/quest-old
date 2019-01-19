use crate::{Shared, IntoObject, Object};
use crate::parse::{self, Parsable, Parser};

pub use crate::object::typed::Number;

impl Parsable for Number {
	const NAME: &'static str = "Number";
	fn try_parse(parser: &Shared<Parser>) -> parse::Result<Object> {
		let number = Number::parse(parser.read().as_ref());

		if let Some((number, index)) = number {
			let mut parser = parser.write();
			let res = parser.advance(index-1);
			debug_assert_eq!(number, Number::parse(&res).unwrap().0);
			debug!(target: "parser", "Number parsed. chars={:?}", res);
			parse::Result::Ok(number.into_object())
		} else {
			trace!(target: "parser", "No number found. stream={:?}", parser.read().beginning());
			parse::Result::None
		}
	}
}