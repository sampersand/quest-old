use parse::{ParserFn, Stream, Parsable, Token};

use env::Environment;
use obj::{Object, AnyShared, Id, types::{*, opers::*}};
use std::fmt::{self, Debug, Formatter};

pub const ALL_PARSERS: &'static [ParserFn] = &[
	parse_whitespace, parse_comment, parse_eof,
	Number::parse,
	Id::parse,
	Text::parse,
	Block::parse,
	List::parse,
	BlockExec::parse,
	Add::parse, Mul::parse, Endline::parse, Comma::parse, Accessor::parse,
	Assign::parse,
];


fn parse_whitespace(stream: &mut Stream) -> Option<Token> {
	match stream.as_str().find(|c: char| !c.is_whitespace()) {
		Some(0) => None,
		Some(nonwhite_pos) => {
			stream.offset_to(nonwhite_pos); // ignore the whitespace; we dont need it.
			stream.next()
		},
		None => 
			if stream.as_str().chars().next().map(char::is_whitespace).unwrap_or(false) {
				stream.offset_to(1);
				stream.next()
			} else {
				None
			}
	}
}

fn parse_comment(stream: &mut Stream) -> Option<Token> {
	if stream.as_str().find('#').or_else(|| stream.as_str().find("//"))? == 0 {
		let newline_pos = stream.as_str().find('\n').unwrap_or_else(|| stream.as_str().len());
		stream.offset_to(newline_pos); // ignore the comment. its not needed
		stream.next()
	} else {
		None
	}
}

fn parse_eof(stream: &mut Stream) -> Option<Token> {
	if stream.as_str().starts_with("__EOF__") || stream.as_str().starts_with("__END__") {
		stream.eof = true;
	}
	return None;
}






