use crate::Shared;
use crate::collections::{Mapping, ParentalMap};
use crate::object::{Object, Type, IntoObject, r#type::Map};
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable(String);

impl Variable {
	pub fn r#static(id: &'static str) -> Variable {
		Variable::new(id.to_string())
	}

	pub fn new(id: String) -> Variable {
		Variable(id)
	}
}


impl From<&'static str> for Variable {
	fn from(id: &'static str) -> Variable {
		Variable::r#static(id)
	}
}

impl From<&'static str> for Object {
	fn from(id: &'static str) -> Object {
		Object::new(Variable::from(id))
	}
}

impl Type for Variable {
	fn create_map() -> Shared<dyn Mapping> {
		lazy_static! {
			static ref CLASS: Shared<Object> = {
				let mut m = Map::empty();
				// m.set("+", unimplemented!())
				m
			}.into_shared();
		}

		Shared::new(ParentalMap::new(CLASS.clone())) as _
		// Shared::new(PArental
		// 	let mut m = PArental::empty();
		// 	m.set("@parent".into_shared(), CLASS.clone());
		// 	m
		// }) as _
	}
}
