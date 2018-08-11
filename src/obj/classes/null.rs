use std::str::FromStr;
use regex::Regex;

use obj::QObject;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QNull;

impl From<()> for QNull {
	fn from(_: ()) -> QNull {
		QNull
	}
}

impl From<()> for QObject {
	#[inline]
	fn from(_: ()) -> QObject {
		QNull.into()
	}
}

impl QNull {
	pub fn new() -> QNull {
		QNull
	}
}

lazy_static! {
	pub static ref RE_NULL: Regex = regex!(r"\A(null|nil|none|NULL|NIL|NONE)\b");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoMatches;

impl FromStr for QNull {
	type Err = NoMatches;
	fn from_str(inp: &str) -> Result<QNull, NoMatches> {
		if RE_NULL.is_match(inp) {
			Ok(QNull)
		} else {
			Err(NoMatches)
		}
	}
}

impl Display for QNull {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "null")
	}
}


default_attrs! { for QNull, with variant Null;
	use QObj;

	fn "@bool" (this) {
		Ok(false.into())
	}
}