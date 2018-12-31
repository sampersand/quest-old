use super::{TypedObject, Type, Types};
use crate::Shared;
use crate::env::Environment;
use crate::object::{Object, IntoObject};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;
use std::fmt::{self, Debug, Display, Formatter};
use std::sync::RwLock;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
	}

	pub fn into_inner(self) -> &'static str {
		self.0
	}
}

impl Debug for Var {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Var({:?})", self.0)
	}
}

impl Display for Var {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl_typed_conversion!(Var, &'static str);
impl_typed_object!(Var, new_var, downcast_var, is_var);
impl_quest_conversion!("@var" (as_var_obj is_var) (into_var downcast_var) -> Var);

impl Object {
	pub fn is_variable(&self, var: &'static str) -> bool {
		if let Some(varobj) = self.downcast_var() {
			varobj.0 == var
		} else {
			false
		}
	}
}

fn env() -> Shared<Environment> {
	Environment::current()
}

impl_type! { for Var, downcast_fn=downcast_var;
	fn "@text" (this) {
		this.0.to_string().into_object()
	}

	fn "@bool" (this) { todo!() }
	
	fn "()" (@this) { env().get(this).unwrap_or_else(Object::new_null) }
	fn "=" (@this, rhs) { env().set(this.clone(), rhs.clone()); rhs.clone() }
	fn "~" (@this) { env().del(this).unwrap_or_else(Object::new_null) }
	fn "?" (@this) { env().has(this).into_object() }
}





