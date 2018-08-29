use env::{Environment, parse::{Parsable, Token}};
use obj::{Id, AnyObject, SharedObject};

use shared::SharedMap;

use std::fmt::{self, Display, Formatter};	
use std::sync::Weak;

pub type QVar = SharedObject<Id>;

impl QVar {
	pub fn from_nonstatic_str(str_id: &str) -> QVar {
		Id::from_nonstatic_str(str_id).into()
	}
}

impl From<&'static str> for QVar {
	#[inline]
	fn from(inp: &'static str) -> QVar {
		Id::from(inp).into()
	}
}

impl Parsable for Id {
	fn try_parse(env: &mut Environment) -> Option<Token> {
		let variable = env.stream.try_get(regex!(r"\A(\$\W|(\$.|[A-Za-z_])\w*)\b"))?;
		Some(Id::from_nonstatic_str(variable).into())
	}
}

define_attrs! { for QVar;
	use QObject<Id>;

	fn "@var" (this) {
		Ok(QVar::from(this.clone()))
	}

	fn "@text" (this) {
		Ok(QText::from(this.as_ref().try_as_str().expect("All Ids should have a str associated with them?").to_string()))
	}
}

