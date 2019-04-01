mod parsable;
mod parser;
pub mod error;

use self::parser::Parser;
use self::parsable::{Parsables, Parsable, ParseResult};

pub use self::error::ParseError;

use std::path::Path;
use crate::error::Result;
use crate::env;
use crate::object::{AnyObject, Object, types, literals};

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<AnyObject> {
	// parser::from_path(path).parse()
	unimplemented!()
}

pub fn parse_str(inp: &str, parsers: Option<Parsables>) -> Result<AnyObject> {
	parse(Parser::from_str(inp, parsers))
}

fn parse(parser: Object<Parser>) -> Result<AnyObject> {
	let env = Object::new(types::Null).as_any();

	env.call_attr(literals::ATTR_SET, &[
		&Object::new_variable(literals::L_PARSER).as_any(),
		&parser.clone().as_any()
	])?;

	env.call_attr(literals::ATTR_SET, &[
		&Object::new_variable(literals::L_STACK).as_any(),
		&Object::new_list(Vec::new()).as_any()
	])?;

	env::push_environment(env);

	parser.parse()
}