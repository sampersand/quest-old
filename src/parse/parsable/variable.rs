use crate::{Shared, Object, IntoObject};
use crate::parse::{self, Parsable, Parser};

pub use crate::object::typed::Variable;

impl Parsable for Variable {
	const NAME: &'static str = "Variable";
	fn try_parse(parser: &Shared<Parser>) -> parse::Result<Object> {
		// let (variable, index) = ParseResult::try_from(Variable::from_str(parser.read().as_ref()))?;
		unimplemented!()
		// match Variable::parse(parser.read().as_ref()) {
		// 	ParseResult::Err(err) => {
		// 	//	...
		// 		unimplemented!()
		// 	},
		// 	o @ ParseResult::None | o @ ParseResult::Ok => o
		// 	o @ ParseResult::Eof | o @ ParseResult::Restart => {
		// 		warn!(target: "parser", "Variable parser returned unexpected result: {:?}", o);
		// 		return o;
		// 	}
		// }
		// let variable = Variable::parse(parser.read().as_ref());

		// if let Some((variable, index)) = variable {
		// 	let mut parser = parser.write();
		// 	let res = parser.advance(index-1);
		// 	debug_assert_eq!(variable, Variable::parse(&res).unwrap().0);
		// 	debug!(target: "parser", "Variable parsed. chars={:?}", res);
		// 	ParseResult::Ok(variable.into_object())
		// } else {
		// 	trace!(target: "parser", "No variable found. stream={:?}", parser.read().beginning());
		// 	ParseResult::None
		// }
	}
}