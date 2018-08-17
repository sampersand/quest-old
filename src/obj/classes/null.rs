use env::{Environment, parse::{Parsable, Token}};
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

impl Parsable for Null {
	fn try_parse(env: &mut Environment) -> Option<Token>{
		env.stream.try_get("null").map(|_| Null.into())
	}
}

define_attrs! { for QNull;
	use QObject<Null>;

	fn "@bool" () {
		Ok(QBool::from(false))
	}
}