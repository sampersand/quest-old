mod stream;
mod funcs;
mod impls;
mod tree;


pub use self::tree::Tree;
pub use self::stream::Stream;

pub(crate) use self::funcs::parse_stream;
pub use self::funcs::{parse_file, parse_str};


use obj::classes::QuestClass;

pub trait Parsable : Sized {
	type Value: QuestClass;
	fn try_parse(stream: &mut Stream) -> Option<Self::Value>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Parser;