use std::sync::RwLock;
use crate::object::{Object, Type};
use std::collections::{HashSet, HashMap};
use crate::{map::Map, shared::Shared};
use lazy_static::lazy_static;



lazy_static::lazy_static! {
	static ref ID_STRINGS: RwLock<HashSet<&'static str>> = RwLock::new(HashSet::new());
}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct Variable(&'static str);

impl Variable {
	#[inline]
	pub fn new(id: &'static str) -> Variable {
		// maybe add the id to ID_STRINGS?
		Variable(id)
	}
}

impl Object<Variable> {
	pub fn new_variable(id: &'static str) -> Object<Variable> {
		Object::new(Variable::new(id))
	}
}


impl From<Variable> for &'static str {
	fn from(var: Variable) -> &'static str {
		var.0
	}
}

impl AsRef<&'static str> for Variable {
	fn as_ref(&self) -> &&'static str {
		&self.0
	}
}

impl From<&'static str> for Variable {
	fn from(id: &'static str) -> Variable {
		Variable::new(id)
	}
}

impl From<String> for Variable {
	fn from(string: String) -> Variable {
		let id_strings = ID_STRINGS.read().expect("ID_STRINGS is poisoned");
		if let Some(id) = id_strings.get(string.as_str()) {
			return Variable::new(id);
		}
		drop(id_strings);
		let mut id_strings = ID_STRINGS.write().expect("ID_STRINGS is poisoned");
		if let Some(id) = id_strings.get(string.as_str()) {// double check
			Variable::new(id)
		} else {
			let s = Box::leak(string.into_boxed_str());
			id_strings.insert(s);
			Variable::new(s)
		}
	}
}

impl_type! { for Variable; }



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		assert_eq!(Variable::new("").as_ref(), &"");
		assert_eq!(Variable::new("foobar").as_ref(), &"foobar");
		assert_eq!(Variable::new("a b c d ").as_ref(), &"a b c d ");
		assert_eq!(Variable::new("I ❤️ Quest").as_ref(), &"I \u{2764}\u{fe0f} Quest");
		assert_eq!(Variable::new("🚀s are cool!").as_ref(), &"\u{1f680}s are cool!");
		assert_eq!(Variable::new("Ɣ㠨𥊗").as_ref(), &"\u{194}㠨\u{25297}");
	}

	#[test]
	fn from_static_str() {
		assert_eq!(Variable::from("foobarbaz").as_ref(), &"foobarbaz");
		assert_eq!(Variable::from("__!_@#__$*!~").as_ref(), &"__!_@#__$*!~");
		assert_eq!(Variable::from("lol").as_ref(), &"lol");
		assert_eq!(Variable::from("I ❤️ 🚀, they r cool").as_ref(), &"I \u{2764}\u{fe0f} \u{1f680}, they r cool");
		assert_eq!(Variable::from("Ɣ㠨𥊗").as_ref(), &"\u{194}㠨\u{25297}");
	}
	#[test]
	fn from_string() {
		assert_eq!(Variable::from("lol".to_string()).as_ref(), &"lol");
		assert_eq!(Variable::from("".to_string()).as_ref(), &"");
		assert_eq!(Variable::from("AWF_EAW".to_string()).as_ref(), &"AWF_EAW");
		assert_eq!(Variable::from("┌─┘lol what ⓪❼༩".to_string()).as_ref(), &"┌─┘lol what ⓪❼༩");
	}

	#[test]
	fn new_variable() {
		assert_eq!(Object::new(Variable::new("quest is fun")), Object::new_variable("quest is fun"));
	}
}