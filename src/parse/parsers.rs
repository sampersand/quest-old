use parse::{ParserFn, Stream, Parsable, ParseResult};

use env::Environment;
use obj::{AnyShared, Id, types::*};
use std::fmt::{self, Debug, Formatter};

pub const ALL_PARSERS: &'static [ParserFn] = &[
	parse_whitespace, parse_comment, parse_eof,
	Number::parse, Id::parse, Text::parse, Oper::parse, Block::parse
];


fn parse_whitespace(stream: &mut Stream, env: &Environment) -> ParseResult {
	match stream.find(|c: char| !c.is_whitespace())? {
		0 => None,
		nonwhite_pos => {
			stream.offset_to(nonwhite_pos); // ignore the whitespace; we dont need it.
			stream.iter(env).next()
		}
	}
}

fn parse_comment(stream: &mut Stream, env: &Environment) -> ParseResult {
	if stream.find('#').or_else(|| stream.find("//"))? == 0 {
		let newline_pos = stream.find('\n').unwrap_or_else(|| stream.len());
		stream.offset_to(newline_pos); // ignore the comment. its not needed
		stream.iter(env).next()
	} else {
		None
	}
}

fn parse_bracket(stream: &mut Stream, _: &Environment) -> ParseResult {
	if stream.starts_with(|c| c == ')' || c == ']' || c == '}') {
		stream.eof = true;
	}
	None // we will always be at end of stream
}

fn parse_eof(stream: &mut Stream, _: &Environment) -> ParseResult {
	if stream.find("__EOF__").or_else(|| stream.find("__END__"))? == 0 {
		stream.eof = true;
		// let len = stream.len();
		// stream.offset_to(len);
	}
	None // we will always be at end of stream
}



