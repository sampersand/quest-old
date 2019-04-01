use crate::object::{Object, AnyObject, literals};
use crate::error::{Result, Error};
use crate::parse::{ParseError, Parsables, ParseResult};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Parser {
	loc: Location,
	data: String,
	parsers: Parsables
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location {
	filename: Option<PathBuf>,
	line: usize,
	col: usize
}

impl Parser {
	pub fn from_str(data: &str, parsers: Option<Parsables>) -> Object<Parser> {
		let parsers = parsers.unwrap_or_else(super::parsable::_default_parsers);

		Object::new(Parser {
			loc: Location { filename: None, line: 0, col: 0 },
			data: data.to_string(),
			parsers
		})
	}

	pub fn advance(&mut self, amnt: usize) -> String {
		let data: String = self.data.drain(..=amnt).collect();
		self.loc.line += data.lines().count();
		self.loc.col = data.lines().last().map(str::len).unwrap_or(0);
		data
	}
}

impl AsRef<str> for Parser {
	fn as_ref(&self) -> &str {
		self.data.as_ref()
	}
} 

impl Object<Parser> {
	pub fn parse(&self) -> Result<AnyObject> {
		macro_rules! read {
			() => (self.data().read().expect("read err in Parser::parser"))
		}

		println!("{:#?}", crate::env::current().unwrap()
			.call_attr(literals::ATTR_GET, &[&Object::new_variable(literals::L_STACK).as_any()])?
			.id());

		println!("{:#?}", crate::env::current().unwrap()
			.call_attr(literals::ATTR_GET, &[&Object::new_variable(literals::L_STACK).as_any()])?
			.id());

		'outer: loop {
			{if read!().data.is_empty() {
				break;
			}}
			let parsers = {read!().parsers.clone()};
			for parser in parsers {
				match parser.parse(self) {
					ParseResult::Ok(object) => 
						crate::env::current().ok_or_else(|| Error::ParseError(ParseError::NoEnvironmentLeft))?
							.call_attr(literals::ATTR_GET, &[&Object::new_variable(literals::L_STACK).as_any()])?
							.call_attr(literals::L_PUSH, &[&object])?,
					ParseResult::Err(err) => return Err(err),
					ParseResult::None => continue,
					ParseResult::Redo => continue 'outer,
					ParseResult::Eof => break 'outer
				};
			}
		}

		println!("{:#?}", crate::env::current().unwrap()
			.call_attr(literals::ATTR_GET, &[&Object::new_variable(literals::L_STACK).as_any()])?
			.id());

		crate::env::current().ok_or_else(|| Error::ParseError(ParseError::NoEnvironmentLeft))?
			.call_attr(literals::ATTR_GET, &[&Object::new_variable(literals::L_STACK).as_any()])?
			.call_attr(literals::L_POP, &[])
	}
}

impl_type! { for Parser; }








