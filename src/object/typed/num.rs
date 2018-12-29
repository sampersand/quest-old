use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Num(i64);

impl Type for Num {
	fn create_mapping() -> Shared<dyn Mapping> {
		lazy_static! {
			static ref PARENT: Shared<Object> = Shared::new(Object::new({
				fn at_text(args: &[&Shared<Object>]) -> crate::Result {
					Ok(TypedObject::new_text(
						args.get(0).ok_or_else(|| crate::Error::BadArgument("missing first arg", None))?
						   .downcast_num().ok_or_else(|| crate::Error::BadArgument("@text called with non-number argument", Some(args[0].clone())))?
						   .0.to_string()
					).objectify())
				}
				let mut map = crate::collections::Map::default();
				map.set(
					TypedObject::new_var("@text").objectify(),
					TypedObject::new_rustfn("Num.@text", at_text).objectify()
				);
				map
			}));
		}
		Shared::new(ParentalMap::new_default(PARENT.clone()))
	}
}

impl From<i64> for Num {
	fn from(num: i64) -> Num {
		Num(num)
	}
}

impl From<Num> for Types {
	fn from(num: Num) -> Types {
		Types::Num(num)
	}
}


impl TypedObject {
	pub fn new_num<T: Into<Num>>(val: T) -> Self {
		TypedObject::new(val.into())
	}

	pub fn downcast_num(&self) -> Option<&Num> {
		if let Types::Num(ref num) = self.data {
			Some(num)
		} else {
			None
		}
	}

}

impl Shared<Object> {
	/// note: this clones the object
	pub fn downcast_num(&self) -> Option<Num> {
		self.read().map.read()
		    .downcast_ref::<TypedObject>()
		    .and_then(TypedObject::downcast_num)
		    .cloned()
	}
}