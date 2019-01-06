use crate::{Shared, Error};
use crate::parse::{Parsable, ParseResult, Parser};

pub(super) struct Comments; 

impl Parsable for Comments {
	const NAME: &'static str = "Comments";
	fn try_parse(parser: &Shared<Parser>) -> ParseResult {
		let (single, multi) = {
			let parser_read = parser.read();
			let data = parser_read.as_ref();

			let single_line = data.starts_with("//") || data.starts_with('#');
			let multi_line = data.starts_with("/*");
			(single_line, multi_line)
		};

		debug_assert!(!(single && multi), "Both single and multiline comments were found?");

		if single {
			let mut parser = parser.write();

			if let Some((idx, _)) = parser.as_ref().chars().enumerate().find(|(_, c)| *c == '\n') {
				let comment = parser.advance(idx);
				debug_assert!(comment.starts_with("//") || comment.starts_with("#"));
				debug_assert!(comment.ends_with('\n'));
				debug!(target: "parser", "Single-line comment parsed. chars={:?}", comment);
				ParseResult::Restart
			} else {
				debug!(target: "parser", "Single-line comment until EOF parsed. chars={:?}", parser.as_ref());
				ParseResult::Eof
			}
		} else if multi {
			let mut parser = parser.write();
			// [2..] skips the `/*` we just found, so `/*/` doesn't work
			if let Some(mut index) = parser.as_ref()[2..].find("*/") {
				let comment = parser.advance(index + 2); // add two to index to make up for slicing `/*` off.
				debug_assert!(comment.starts_with("/*"));
				debug_assert!(comment.ends_with("*/"));
				debug_assert!(comment.len() >= 4);
				debug!(target: "parser", "Multi-line comment parsed. chars={:?}", comment);
				ParseResult::Restart
			} else {
				warn!(target: "parser", "No ending `*/` found for multiline comment. data={:?}", parser.beginning());
				ParseResult::Eof
			}
		} else {
			trace!(target: "parser", "No comment found. stream={:?}", parser.read().beginning());
			ParseResult::None
		}
	}
}




