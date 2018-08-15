use env::{Environment, Stream, Parsable, Token};
use obj::AnyObject;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Whitespace;

impl Parsable for Whitespace {
	fn try_parse(&self, env: &mut Environment) -> Option<Token> {
		env.stream.try_get(regex!(r"\A\s+")).and(Some(Token::NoObject))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Comment;

impl Parsable for Comment {
	fn try_parse(&self, env: &mut Environment) -> Option<Token> {
		env.stream.try_get(regex!(r"(?m)\A(//|#).*$")).and(Some(Token::NoObject))
	}
}