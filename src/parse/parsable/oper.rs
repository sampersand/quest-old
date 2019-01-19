use crate::{Shared, IntoObject, Object};
use crate::parse::{self, Parsable, Parser};

pub use crate::object::typed::Oper;

// in the future, i might add a parsable for each oper to allow for adding / subtraction operators
// but for now, just to get this working, one parsable for the entire oper
impl Parsable for Oper {
	const NAME: &'static str = "Oper";
	fn try_parse(parser: &Shared<Parser>) -> parse::Result<Object> {
		let oper = Oper::parse(parser.read().as_ref());

		if let Some((oper, index)) = oper {
			let res = parser.write().advance(index-1);
			debug_assert_eq!(oper, Oper::parse(&res).unwrap().0);
			debug!(target: "parser", "Oper parsed. chars={:?}", res);
			parse::Result::Ok(oper.into_object())
		} else {
			trace!(target: "parser", "No oper found. stream={:?}", parser.read().beginning());
			parse::Result::None
		}
	}
}