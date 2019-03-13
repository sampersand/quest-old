use std::sync::RwLock;
use crate::object::{Object, AnyObject, Type};
use std::collections::{HashSet, HashMap};
use crate::{map::Map, shared::Shared};
use std::ops::Deref;
use crate::err::Result;

use crate::object::{Literal, literals};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Variable(Literal);

impl Variable {
	#[inline]
	pub fn new(id: Literal) -> Variable {
		Variable(id)
	}
}

impl Object<Variable> {
	pub fn new_variable(id: Literal) -> Object<Variable> {
		Object::new(Variable::from(id))
	}

	pub fn new_variable_from_string(id: String) -> Object<Variable> {
		Object::new(Variable::from(id))
	}

	#[cfg(test)]
	pub fn new_variable_testing(s: &'static str) -> Object<Variable> {
		Object::new_variable(Literal::new_testing(s))
	}
}

impl AnyObject {
	pub fn to_variable(&self) -> Result<Object<Variable>> {
		self.call_attr(literals::AT_VAR, &[])?
			.downcast_or_err::<Variable>()
	}
}

impl Deref for Variable {
	type Target = Literal;
	fn deref(&self) -> &Literal {
		&self.0
	}
}

impl From<Variable> for Literal {
	fn from(var: Variable) -> Literal {
		var.0
	}
}

impl AsRef<Literal> for Variable {
	fn as_ref(&self) -> &Literal {
		&self.0
	}
}

impl From<Literal> for Variable {
	fn from(id: Literal) -> Variable {
		Variable::new(id)
	}
}

impl From<String> for Variable {
	fn from(string: String) -> Variable {
		Variable::new(Literal::from(string))
	}
}

impl PartialEq<Literal> for Object<Variable> {
	fn eq(&self, rhs: &Literal) -> bool {
		self.data().read().expect("read error in Object<Variable>::eq").as_ref() == rhs
	}
}


impl PartialEq<str> for Object<Variable> {
	fn eq(&self, rhs: &str) -> bool {
		*self.data().read().expect("read error in Object<Variable>::eq").as_ref() == rhs
	}
}

impl_type! { for Variable;
	literals::AT_VAR => |obj, _| Ok(Object::new_variable(**obj.data().read().expect(data_err![read in Variable, literals::AT_VAR])))
}



#[cfg(test)]
mod tests {
	use super::*;
	macro_rules! l {
		($l:expr) => { Literal::new_testing($l) }
	}

	#[test]
	fn new() {
		assert_eq!(*Variable::new(l![""]), l![""]);
		assert_eq!(*Variable::new(l!["foobar"]), l!["foobar"]);
		assert_eq!(*Variable::new(l!["a b c d "]), l!["a b c d "]);
		assert_eq!(*Variable::new(l!["`I ❤️ Quest`"]), l!["`I \u{2764}\u{fe0f} Quest`"]);
		assert_eq!(*Variable::new(l!["🚀s are cool!"]), l!["\u{1f680}s are cool!"]);
		assert_eq!(*Variable::new(l!["Ɣ㠨𥊗"]), l!["\u{194}㠨\u{25297}"]);
	}

	#[test]
	fn from_static_str() {
		assert_eq!(*Variable::from(l!["foobarbaz"]), l!["foobarbaz"]);
		assert_eq!(*Variable::from(l!["__!_@#__$*!~"]), l!["__!_@#__$*!~"]);
		assert_eq!(*Variable::from(l!["`lol`"]), l!["`lol`"]);
		assert_eq!(*Variable::from(l!["I ❤️ 🚀, they r cool"]), l!["I \u{2764}\u{fe0f} \u{1f680}, they r cool"]);
		assert_eq!(*Variable::from(l!["Ɣ㠨𥊗"]), l!["\u{194}㠨\u{25297}"]);
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
		assert_eq!(Object::new(Variable::new(l!["`quest is fun`"])), Object::new_variable(l!["`quest is fun`"]));
	}

	#[test]
	fn to_variable() -> Result<()> {
		assert_eq!(*Object::new_variable(l!["abc"]).as_any().to_variable()?.unwrap_data(), "abc");
		assert_eq!(*Object::new_variable(l![""]).as_any().to_variable()?.unwrap_data(), "");
		assert_eq!(*Object::new_variable(l!["I ❤️ 🚀, they r cool"]).as_any().to_variable()?.unwrap_data(), "I ❤️ 🚀, they r cool");
		
		Ok(())
	}
}