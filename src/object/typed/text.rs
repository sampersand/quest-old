use super::{TypedObject, Type, Types};
use crate::Shared;
use crate::object::{Object, IntoObject};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Text(String);

// impl Type for Text {
// 	fn create_mapping() -> Shared<dyn Mapping> {
// 		lazy_static! {
// 			static ref PARENT: Object = Shared::new({
// 				ObjectInner::new(crate::collections::Map::default())
// 			});
// 		}
// 		Shared::new(ParentalMap::new_default(|| PARENT.clone()))
// 	}
// }

impl IntoObject for String {
	fn into_object(self) -> Object {
		TypedObject::new_text(self).objectify()
	}
}

impl From<String> for Text {
	fn from(text: String) -> Text {
		Text(text)
	}
}

impl_typed_object!(Text, new_text, downcast_text);


impl_type! { for Text, downcast_fn=downcast_text;
	fn "@var" (this) {
		Var::from_string(this.0).into_object()
	}
}








