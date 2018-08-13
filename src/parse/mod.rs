mod stream;
mod funcs;
mod impls;

pub use self::stream::Stream;
pub use self::funcs::{parse_file};


use obj::classes::QuestClass;

pub trait Parsable : Sized {
	type Value: QuestClass;
	fn try_parse(stream: &mut Stream) -> Option<Self::Value>;
}

