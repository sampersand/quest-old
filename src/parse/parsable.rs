macro_rules! named {
	($ty:ty) => (impl $crate::parse::parsable::Named for $ty {
		const NAME: &'static str = stringify!($ty);
	})
}

mod r#struct;
mod whitespace;
mod forced_eof;
mod comments;
mod number;
mod variable;
mod text;
mod oper;
mod block;

use crate::{Shared, Object, Error};
use crate::parse::{self, Parser};
use lazy_static::lazy_static;

// A hack to get `NAME` working
pub trait Named {
	const NAME: &'static str;
}

pub trait ParseFromStr : Sized {
	type Err: std::error::Error + 'static;
	fn from_str(inp: &str) -> Result<ParseOk<Self>, Self::Err>;
}

pub enum ParseOk<T> {
	Found(T, usize),
	NotFound
}

impl<T> From<std::option::NoneError> for ParseOk<T> {
	fn from(_: std::option::NoneError) -> ParseOk<T> {
		ParseOk::NotFound
	}
}

pub trait Parsable : Named {
	fn try_parse(parser: &Shared<Parser>) -> parse::Result<Object>;
}

impl<T: ParseFromStr + Named + crate::object::IntoObject> Parsable for T {
	fn try_parse(parser: &Shared<Parser>) -> parse::Result<Object> {
		let parse_result = Self::from_str(parser.read().as_ref());
		match parse_result {
			Ok(ParseOk::NotFound) => {
				trace!(target: "parser", "{} wasn't found. stream={:?}", Self::NAME, parser.read().beginning());
				parse::Result::None
			},
			Ok(ParseOk::Found(object, index)) => {
				let mut parser = parser.write();
				let res = parser.advance(index-1);
				// debug_assert_eq!(number, Number::parse(&res).unwrap().0);
				debug!(target: "parser", "{} parsed. chars={:?}", Self::NAME, res);
				parse::Result::Ok(object.into_object())
			},
			Err(err) => {
				warn!(target: "parser", "{} parsing caused an error. err={:?}", Self::NAME, err);
				parse::Result::Err(Box::new(err))
			}
		}
	}
} 

pub use self::r#struct::ParsableStruct;

lazy_static! {
	pub static ref BUILTIN_PARSERS: Shared<Vec<ParsableStruct>> = Shared::new(vec!{
		ParsableStruct::new::<whitespace::Whitespace>(),
		ParsableStruct::new::<forced_eof::ForcedEof>(),
		ParsableStruct::new::<comments::Comments>(),
		ParsableStruct::new::<number::Number>(),
		ParsableStruct::new::<text::Text>(),
		ParsableStruct::new::<oper::Oper>(),
		ParsableStruct::new::<variable::Variable>(),
		ParsableStruct::new::<block::Block>(),
	});
}