use super::{TypedObject, Type, Types};
use crate::Shared;
use crate::object::{Object, IntoObject};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;
use std::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Var(&'static str);

lazy_static! {
	static ref ID_STRINGS: RwLock<Vec<&'static str>> = RwLock::new(Vec::new());
}

impl Var {
	pub fn new(id: &'static str) -> Var {
		// maybe add the id to ID_STRINGS?
		Var(id)
	}

	pub fn from_string(string: String) -> Var {
		let id_strings = ID_STRINGS.read().expect("ID_STRINGS is poisoned");
		if let Some(index) = id_strings.iter().position(|x| x == &string) {
			Var(id_strings[index])
		} else {
			drop(id_strings);
			let mut id_strings = ID_STRINGS.write().expect("ID_STRINGS is poisoned");
			if id_strings.contains(&string.as_str()) {
				drop(id_strings);
				Var::from_string(string)
			} else {
				let s = Box::leak(string.into_boxed_str());
				id_strings.push(s);
				Var(id_strings[id_strings.len() - 1])
			}
		}
		// if id_strings.contains(string.as_str()) {

		// 	Var::new(id_strings.index(string.as_str()))
		// }
		// unimplemented!()
	}
}

impl IntoObject for &'static str {
	fn into_shared(self) -> Shared<Object> {
		TypedObject::new_var(self).objectify()
	}
}

impl From<&'static str> for Var {
	fn from(id: &'static str) -> Var {
		Var(id)
	}
}

impl_typed_object!(Var, new_var, downcast_var);


impl_type! { for Var, downcast_fn=downcast_var;
	fn "@text" (this) {
		this.0.to_string().into_shared()
	}
}





