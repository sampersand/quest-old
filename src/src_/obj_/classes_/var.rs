use regex::Regex;
use obj_::{Id, QObject__};

use std::ops::Deref;
use std::str::FromStr;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct QVar(Id);

impl QVar {
	#[inline]
	pub fn new<I: Into<Id>>(id: I) -> QVar {
		QVar::from(id.into())
	}

	pub fn from_nonstatic_str(id: &str) -> QVar {
		Id::from_nonstatic_str(id).into()
	}

	pub fn as_id(&self) -> Id {
		self.0
	}
}


impl From<&'static str> for QVar {
	#[inline]
	fn from(id: &'static str) -> QVar {
		QVar::from(Id::from(id))
	}
}


impl From<&'static str> for QObject__ {
	#[inline]
	fn from(id: &'static str) -> QObject__ {
		QVar::from(id).into()
	}
}


impl From<Id> for QObject__ {
	#[inline]
	fn from(id: Id) -> QObject__ {
		QVar::from(id).into()
	}
}

impl From<Id> for QVar {
	#[inline]
	fn from(id: Id) -> QVar {
		QVar(id)
	}
}

impl From<QVar> for Id {
	#[inline]
	fn from(var: QVar) -> Id {
		var.0
	}
}

impl AsRef<Id> for QVar {
	fn as_ref(&self) -> &Id {
		&self.0
	}
}

impl Deref for QVar {
	type Target = Id;

	#[inline]
	fn deref(&self) -> &Id {
		&self.0
	}
}

impl Display for QVar {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "`{}`", self.0)
	}
}

impl Debug for QVar {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "QVar({})", self.0)
	}
}


lazy_static! {
	pub static ref REGEX: Regex = regex!(r"\A(?:(?:[$@].|[a-zA-Z_])\w*(?:[?!]|\b)|(?:(?:[$@].)))");
	// pub static ref REGEX: Regex = regex!(r"\A(?:[$@].|[a-zA-Z_]\w*(?:[?!]|\b))");
}

impl FromStr for QVar {
	type Err = !;
	fn from_str(s: &str) -> Result<QVar, !> {
		Ok(QVar::from_nonstatic_str(s))
	}
}

default_attrs! { for QVar, with variant Var;
	use QObj;
	fn "@var" (this) {
		Ok(::obj_::QObject_::Old(this.clone().into()))
	}

	fn "@text" (this) {
		Ok(::obj_::QObject_::Old(this.0.str_id().expect("all vars should have id strs associated?").to_string().into()))
	}
}