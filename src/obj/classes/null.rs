use parse::{Parsable, Stream};
use std::marker::PhantomData;
use obj::object::QObject;
use obj::classes::{QuestClass, DefaultAttrs};
use std::fmt::{self, Display, Formatter};	

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Null;

pub type QNull = QObject<Null>;

impl Display for Null {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "null")
	}
}

impl Parsable for Null {
	type Value = Null;

	fn try_parse(stream: &mut Stream) -> Option<Null> {
		stream.try_get("null").and(Some(Null))
	}
}

impl QuestClass for Null {
	fn default_attrs() -> &'static DefaultAttrs<Self> { &DEFAULT_ATTRS }
}


define_attrs! {
	static ref DEFAULT_ATTRS for Null;
	use QObject<Null>;

	fn "@num" () {
		Ok(QBool::from(false))
	}
}
