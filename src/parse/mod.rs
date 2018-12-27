mod tree;
mod stream;
mod parsers;
mod token;
mod opers;

pub use self::tree::Tree;
pub use self::token::{Token, Precedence};
pub use self::stream::{Stream, ParserFnVec};

pub fn default_parsers() -> Vec<ParserFn> {
	parsers::ALL_PARSERS.to_vec()
}

pub trait Parsable {
	fn parse(stream: &mut Stream) -> Option<Token>;
}

type ParserFn = fn(&mut Stream) -> Option<Token>;