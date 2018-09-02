mod tree;
mod stream;
mod parsers;


pub type ParseResult = Option<Box<dyn (::obj::types::IntoAnyObject)>>;

type ParserFn = fn(&mut Stream, &::env::Environment) -> ParseResult;

pub trait Parsable {
	fn parse(&mut Stream, &::env::Environment) -> ParseResult;
}

pub use self::tree::Tree;
pub use self::stream::{Stream, StreamChars};

pub fn default_parsers() -> Vec<ParserFn> {
	parsers::ALL_PARSERS.to_vec()
}