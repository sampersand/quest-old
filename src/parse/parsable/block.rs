use crate::{Shared, IntoObject, Object};
use crate::parse::{self, Parser, Parsable};
use crate::parse::parsable::{ParseFromStr, ParseOk, Named};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub use crate::object::typed::Block;
use crate::object::typed::block::Parens;

named!(Block);


impl Parsable for Block {
	fn try_parse(parser: &Shared<Parser>) -> parse::Result<Object> {
		let first = if let Some(chr) = parser.read().as_ref().chars().next() {
			chr
		} else {
			return parse::Result::None;
		};

		// if it's an ending paren, we're at eof
		if let Some(paren) = Parens::try_from_end(first) {
			let x = parser.write().advance(0);
			debug_assert_eq!(Parens::try_from_end(x.chars().next().unwrap()), Some(paren));
			 // we know its the end of block because it's eof
			// so if we get an ending, we are at end of block
			return parse::Result::Eof;
		}

		// if it's a starting paren, go until we hit eof
		let paren = if let Some(paren) = Parens::try_from_start(first) {
			let x = parser.write().advance(0);
			debug_assert_eq!(Parens::try_from_start(x.chars().next().unwrap()), Some(paren));
			paren
		} else {
			return parse::Result::None;
		};

		let mut block = String::new();
		let old_chars = parser.read().location().chars;
		let data = { parser.read().as_ref().to_owned() }; // this is so inefficient right here

		loop {
			match Parser::next_unevaluated_object(parser) {
				None => break,
				Some(Ok(obj)) => { /* do nothing*/ },
				Some(Err(err)) => return parse::Result::Err(Box::new(err))
			}
		}
		// now we've hit EOF
		let chars_eaten = parser.read().location().chars - old_chars;
		let mut body = data.chars().take(chars_eaten).collect::<String>();
		if body.chars().last().and_then(Parens::try_from_end).is_some() {
			body.pop();
		}

		parse::Result::Ok(Block::new(paren, body).into_object())
		// for chr in chars {
		// 	if chr == '\\' {
		// 		block 
		// 	}
		// }

	// pub fn parse(text: &str) -> Option<(Block, usize)> {
	// 	// todo: parse block
	// 	if text.starts_with('{') {
	// 		let index = text.find('}').expect("Bad index");
	// 		let body: String = text.chars().skip(1).take(index-1).collect();
	// 		Some((Block::new(Parens::Curly, body), index + 1))
	// 	} else {
	// 		None
	// 	}
	// }
	}
}
