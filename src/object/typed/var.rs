use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Var(String);

impl Type for Var {
	fn create_mapping() -> Shared<dyn Mapping> {
		lazy_static! {
			static ref PARENT: Shared<Object> = Shared::new({
				unimplemented!();
			});
		}
		Shared::new(ParentalMap::new_default(PARENT.clone()))
	}
}

impl From<&'static str> for Var {
	fn from(id: &'static str) -> Var {
		Var(id.to_string())
	}
}

impl From<Var> for Types {
	fn from(id: Var) -> Types {
		Types::Var(id)
	}
}


impl TypedObject {
	pub fn new_var<T: Into<Var>>(val: T) -> Self {
		TypedObject::new(val.into())
	}
}