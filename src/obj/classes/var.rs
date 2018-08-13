use parse::{Parsable, Stream};

use shared::SharedMap;
use obj::{Id, object::QObject};
use obj::classes::{QuestClass, DefaultAttrs};

use std::fmt::{self, Display, Formatter};	
use std::sync::Weak;

pub type QVar = QObject<Id>;

impl QVar {
	pub fn from_nonstatic_str(str_id: &str) -> QVar {
		QVar::new(Id::from_nonstatic_str(str_id))
	}
}

impl From<&'static str> for QVar {
	#[inline]
	fn from(inp: &'static str) -> QVar {
		QVar::new(inp.into())
	}
}

impl Parsable for Id {
	type Value = Id;

	fn try_parse(stream: &mut Stream) -> Option<Id> {
		let variable = stream.try_get(regex!(r"\A(\$\W|(\$.|[A-Za-z_])\w+)\b"))?;
		Some(Id::from_nonstatic_str(variable)) 
	}
}


impl QuestClass for Id {
	fn default_attrs() -> &'static DefaultAttrs<Self> { &DEFAULT_ATTRS }
}

define_attrs! {
	static ref DEFAULT_ATTRS for Id;
	use QObject<Id>;

	fn "@var" (this) {
		Ok(this.clone())
	}

	fn "@text" (this) {
		Ok(QText::from(this.as_ref().try_as_str().expect("All Ids should have a str associated with them?").to_string()))
	}
}

