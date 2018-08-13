use parse::{Parsable, Stream};
use obj::{AnyObject, SharedObject};

use std::fmt::{self, Display, Formatter};	

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Null;

pub type QNull = SharedObject<Null>;

impl Display for Null {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "null")
	}
}

impl Parsable for QNull {
	type Value = QNull;

	fn try_parse(stream: &mut Stream) -> Option<QNull> {
		stream.try_get("null").map(|_| Null.into())
	}
}

define_attrs! {
	static ref DEFAULT_ATTRS for QNull;
	use QObject<Null>;

	fn "@num" () {
		Ok(QBool::from(false))
	}
}
