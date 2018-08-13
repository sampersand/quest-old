use parse::{Parsable, Stream};

use obj::object::QObject;
use obj::classes::{QuestClass, DefaultAttrs};

use std::fmt::{self, Display, Formatter};

pub type QBool = QObject<bool>;

impl Display for QBool {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(self.as_ref(), f)
	}
}

impl Parsable for bool {
	type Value = bool;

	fn try_parse(stream: &mut Stream) -> Option<bool> {
		match stream.try_get(regex!(r"\A([tT]rue|[fF]alse)\b"))? {
			"true"  | "True"  => Some(true),
			"false" | "False" => Some(false),
			other => unreachable!("found non-bool regex value `{:?}`", other)
		}
	}
}


impl QuestClass for bool {
	fn default_attrs() -> &'static DefaultAttrs<Self> { &DEFAULT_ATTRS }
}

define_attrs! {
	static ref DEFAULT_ATTRS for bool;
	use QObject<bool>;

	fn "@num" (this) {
		Ok(QNum::from_number(**this as u8))
	}

	fn "@bool" (this) {
		Ok(this.clone())
	}
}