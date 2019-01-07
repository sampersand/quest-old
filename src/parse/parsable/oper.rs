use crate::{Shared, IntoObject};
use crate::parse::{Parsable, ParseResult, Parser};

pub use crate::object::typed::Oper;

// in the future, i might add a parsable for each oper to allow for adding / subtraction operators
// but for now, just to get this working, one parsable for the entire oper
impl Parsable for Oper {
	const NAME: &'static str = "Oper";
	fn try_parse(parser: &Shared<Parser>) -> ParseResult {
		let oper = Oper::parse(parser.read().as_ref());

		if let Some((oper, index)) = oper {
			let res = parser.write().advance(index-1);
			debug_assert_eq!(oper, Oper::parse(&res).unwrap().0);
			debug!(target: "parser", "Oper parsed. chars={:?}", res);
			ParseResult::Ok(oper.into_object())
		} else {
			trace!(target: "parser", "No oper found. stream={:?}", parser.read().beginning());
			ParseResult::None
		}
	}
}