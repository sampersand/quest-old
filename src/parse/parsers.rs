use parse::{ParserFn, Stream, Parsable, Token, opers::*};

use env::Environment;
use obj::{Object, AnyShared, Id, types::*};
use std::fmt::{self, Debug, Formatter};

pub const ALL_PARSERS: &'static [ParserFn] = &[
	Add::parse, AddAug::parse, Sub::parse, SubAug::parse,
	Mul::parse, MulAug::parse, Div::parse, DivAug::parse, Mod::parse, ModAug::parse,
	Pow::parse, PowAug::parse,

	Lt::parse, Gt::parse, Le::parse, Ge::parse, Eq::parse, Ne::parse, Cmp::parse,
	And::parse, Or::parse, 

	Endline::parse, Comma::parse, Accessor::parse,
	Assign::parse,

	parse_whitespace, parse_comment, parse_eof,

	Number::parse, Text::parse,
	Block::parse, List::parse, BlockExec::parse,
	Var::parse, RawVar::parse
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






