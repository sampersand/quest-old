use crate::{Shared, IntoObject};
use crate::parse::{Parsable, ParseResult, Parser};

pub use crate::object::typed::Variable;

impl Parsable for Variable {
	const NAME: &'static str = "Variable";
	fn try_parse(parser: &Shared<Parser>) -> ParseResult {
		let variable = Variable::parse(parser.read().as_ref());

		if let Some((variable, index)) = variable {
			let mut parser = parser.write();
			let res = parser.advance(index-1);
			debug_assert_eq!(variable, Variable::parse(&res).unwrap().0);
			debug!(target: "parser", "Variable parsed. chars={:?}", res);
			ParseResult::Ok(variable.into_object())
		} else {
			trace!(target: "parser", "No variable found. stream={:?}", parser.read().beginning());
			ParseResult::None
		}
	}
}