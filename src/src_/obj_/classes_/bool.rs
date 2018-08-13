use obj_::{QObject__, classes_::QNum};
use env_::Environment__;
use std::ops::{Deref, DerefMut};
use regex::Regex;
use std::str::FromStr;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QBool(bool);

impl QBool {
	pub fn new(inp: bool) -> QBool {
		QBool(inp)
	}

	pub fn to_bool(&self) -> bool {
		self.0
	}
}

impl Display for QBool {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl From<bool> for QObject__ {
	#[inline]
	fn from(b: bool) -> QObject__ {
		QBool::from(b).into()
	}
}

impl From<bool> for QBool {
	#[inline]
	fn from(b: bool) -> QBool {
		QBool(b)
	}
}

impl From<QBool> for bool {
	#[inline]
	fn from(qb: QBool) -> bool {
		qb.0
	}
}

impl Deref for QBool {
	type Target = bool;
	#[inline]
	fn deref(&self) -> &bool {
		&self.0
	}
}

impl AsRef<bool> for QBool {
	#[inline]
	fn as_ref(&self) -> &bool {
		&self.0
	}
}

lazy_static! {
	pub static ref RE_TRUE: Regex = regex!(r"\A(?:true|TRUE)\b");
	pub static ref RE_FALSE: Regex = regex!(r"\A(?:false|FALSE)\b");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoMatches;

impl FromStr for QBool {
	type Err = NoMatches;
	fn from_str(inp: &str) -> Result<QBool, NoMatches> {
		if RE_TRUE.is_match(inp) {
			Ok(true.into())
		} else if RE_FALSE.is_match(inp) {
			Ok(false.into())
		} else {
			Err(NoMatches)
		}
	}
}

default_attrs!{ for QBool, with variant Bool ;
	use QObj;
	fn "@num" (this) {
		Ok(::obj_::QObject_::Old(QNum::new(this.0 as u8 as _).into()))
	}

	fn "@bool" (this) {
		Ok(::obj_::QObject_::Old(this.clone().into()))
	}
}