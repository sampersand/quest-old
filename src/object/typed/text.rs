use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Text(String);

impl Type for Text {
	fn create_mapping() -> Shared<dyn Mapping> {
		lazy_static! {
			static ref PARENT: Shared<Object> = Shared::new({
				unimplemented!();
			});
		}
		Shared::new(ParentalMap::new_default(PARENT.clone()))
	}
}

impl From<String> for Text {
	fn from(text: String) -> Text {
		Text(text)
	}
}

impl From<Text> for Types {
	fn from(text: Text) -> Types {
		Types::Text(text)
	}
}


impl TypedObject {
	pub fn new_text<T: Into<Text>>(val: T) -> Self {
		TypedObject::new(val.into())
	}
}