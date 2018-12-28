use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Num(i64);

impl Type for Num {
	fn create_mapping() -> Shared<dyn Mapping> {
		lazy_static! {
			static ref PARENT: Shared<Object> = Shared::new({
				Object::new(crate::collections::Map::default())
			});
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
}