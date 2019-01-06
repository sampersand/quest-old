use crate::Shared;
use crate::env::Environment;
use crate::object::Object;
use lazy_static::lazy_static;
use std::fmt::{self, Debug, Display, Formatter};
use std::sync::RwLock;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Variable(&'static str);

lazy_static! {
	static ref ID_STRINGS: RwLock<Vec<&'static str>> = RwLock::new(Vec::new());
}

impl Variable {
	pub fn new(id: &'static str) -> Variable {
		// maybe add the id to ID_STRINGS?
		Variable(id)
	}

	pub fn from_string(string: String) -> Variable {
		let id_strings = ID_STRINGS.read().expect("ID_STRINGS is poisoned");
		if let Some(index) = id_strings.iter().position(|x| x == &string) {
			Variable(id_strings[index])
		} else {
			drop(id_strings);
			let mut id_strings = ID_STRINGS.write().expect("ID_STRINGS is poisoned");
			if id_strings.contains(&string.as_str()) {
				drop(id_strings);
				Variable::from_string(string)
			} else {
				let s = Box::leak(string.into_boxed_str());
				id_strings.push(s);
				Variable(id_strings[id_strings.len() - 1])
			}
		}
	}

	pub fn into_inner(self) -> &'static str {
		self.0
	}
}

impl Debug for Variable {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Variable({:?})", self.0)
	}
}

impl Display for Variable {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl_typed_conversion!(Variable, &'static str);
impl_typed_object!(Variable, new_var, downcast_var, is_var);
impl_quest_conversion!("@var" (as_var_obj is_var) (into_var downcast_var) -> Variable);

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

impl_type! { for Variable, downcast_fn=downcast_var;
	fn "@text" (this) {
		this.0.to_string().into_object()
	}

	fn "@bool" (_this) { todo!() }
	
	fn "==" (this, rhs) {
		(this == rhs.into_var()?).into_object()
	}

	fn "()" (@this) { env().get(this).unwrap_or_else(Object::new_null) }
	fn "=" (@this, rhs) { env().set(this.clone(), rhs.clone()); rhs.clone() }
	fn "~" (@this) { env().del(this).unwrap_or_else(Object::new_null) }
	fn "?" (@this) { env().has(this).into_object() }
}