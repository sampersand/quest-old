use crate::{Shared, IntoObject, Object};
use crate::parse::{self, Parser};
use crate::parse::parsable::{ParseFromStr, ParseOk, CharIter};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub use crate::object::typed::Text;

#[derive(Debug)]
pub enum TextParseError {
	Unterminated,
	BadHexEscapeSeq(Option<char>, Option<char>),
	BadEscapeSeq(char),
}
use self::TextParseError::*;

named!(Text);

fn parse_escape(chars: &mut CharIter<'_>) -> Result<char, TextParseError> {
	match chars.next().ok_or(Unterminated)? {
		c @ '\\' | c @ '\'' | c @ '\"' => Ok(c),
		'n' => Ok('\n'),
		't' => Ok('\t'),
		'r' => Ok('\r'),
		'0' => Ok('\0'),
		'x' => {
			let first = chars.next().ok_or_else(|| BadHexEscapeSeq(None, None))?;
			let second = chars.next().ok_or_else(|| BadHexEscapeSeq(Some(first), None))?;
			first.to_digit(16)
				.and_then(|x| second.to_digit(16).map(|y| (x << 4) + y))
				.and_then(std::char::from_u32)
				.ok_or_else(|| BadHexEscapeSeq(Some(first), Some(second)))
		},
		'u' => unimplemented!("\\u"),
		'U' => unimplemented!("\\U"),
		other => Err(BadEscapeSeq(other))
	}
}

impl ParseFromStr for Text {
	type Err = TextParseError;
	fn from_str(text: &str) -> Result<ParseOk<Text>, TextParseError> {
		let mut chars = CharIter::from(text);

		let quote = match chars.next() {
			Some(q @ '\"') | Some(q @ '\'') => q,
			_ => return Ok(ParseOk::NotFound)
		};

		debug_assert!(quote == '\'' || quote == '\"', quote);
		let mut text = String::new();

		loop {
			match chars.next() {
				Some(q) if q == quote => break,
				Some('\\') => text.push(parse_escape(&mut chars)?),
				Some(other) => text.push(other),
				None => return Err(Unterminated)
			}
		}

		Ok(ParseOk::Found(Text::from(text), chars.chars_count()))
	}
}

impl Display for TextParseError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Unterminated => write!(f, "Unterminated string"),
			BadEscapeSeq(seq) => write!(f, "Bad escape sequence '\\{}'", seq),
			BadHexEscapeSeq(None, None) => write!(f, "Missing hex escape seq"),
			BadHexEscapeSeq(Some(x), None) => write!(f, "Bad hex escape seq '\\x{}'", x),
			BadHexEscapeSeq(Some(x), Some(y)) => write!(f, "Bad hex escape seq '\\x{}{}'", x, y),
			BadHexEscapeSeq(None, Some(_)) => unreachable!(),
		}
	}
}

impl Error for TextParseError {
	fn description(&self) -> &str {
		match self {
			Unterminated => "unterminated string",
			BadEscapeSeq(_) => "bad escape sequence",
			BadHexEscapeSeq(_, _) => "bad hex escape sequence",
		}
	}
}