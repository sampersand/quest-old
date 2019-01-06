pub mod builtins;

use crate::{Shared, Object, parse::Parser};
use crate::collections::{Collection, Mapping, Listing};
use std::fmt::{self, Display, Formatter};
use std::{mem, sync::RwLock};
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Environment {
	parent: Option<Shared<Environment>>,
	parser: Shared<Parser>,
	map: Shared<dyn Mapping>,
	stack: Shared<dyn Listing>
}

impl Environment {
	fn empty() -> Environment {
		Environment {
			parent: None,
			parser: Shared::new(Parser::default()),
			map: Shared::new(crate::collections::Map::empty()),
			stack: Shared::new(crate::collections::List::empty())
		}
	}

	// im not sure how i want initialization to work, that's why this is lower case
	pub fn _new_default_with_stream(parser: Shared<Parser>) -> Shared<Environment> {
		Shared::new(Environment {
			parser,
			map: Shared::new(crate::collections::ParentalMap::new_default(|| builtins::BUILTINS_MAP.clone())),
			..Environment::empty()
		})
	}

	pub fn execute(env: Shared<Environment>) -> crate::Result {
		trace!(target: "execute", "Starting to execute");
		let mut parser = env.read().parser.clone();

		while let Some(object) = Parser::next_object(parser)? {
			trace!(target: "execute", "received next object");
			env.read().stack.write().push(object);
			parser = env.read().parser.clone(); // in case it was updated somehow
		}

		// todo: return an error
		Ok(env.read().stack.write().pop().unwrap_or_else(Object::new_null))
	}
}

/** CURRENT for env **/
lazy_static! {
	static ref CURRENT: RwLock<Shared<Environment>> = RwLock::new(Environment::_new_default_with_stream(Shared::new(Parser::default())));
}

impl Environment {
	pub fn current() -> Shared<Environment> {
		CURRENT.read().expect("current environment unreadable").clone()
	}

	pub fn set_current(mut env: Shared<Environment>) -> Shared<Environment> {
		mem::swap(&mut env, &mut *CURRENT.write().expect("current environment unwritable"));
		// `env` is now the old CURRENT and can be used as such.
		env
	}
	// pub fn push_env(env: Shared<Environment>) ->
}

impl Display for Environment {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "<environment; todo: this>")
	}
}


impl Collection for Environment {
	fn len(&self) -> usize {
		self.map.len()
	}

	fn is_empty(&self) -> bool {
		self.map.is_empty()
	}
}

impl Mapping for Environment {
	fn duplicate(&self) -> Shared<dyn Mapping> {
		unimplemented!("duplicate")
	}

	fn get(&self, key: &Object) -> Option<Object> {
		self.map.get(key)
	}

	fn set(&mut self, key: Object, val: Object) -> Option<Object> {
		self.map.set(key, val)
	}

	fn del(&mut self, key: &Object) -> Option<Object> {
		self.map.del(key)
	}

	fn has(&self, key: &Object) -> bool {
		self.map.has(key)
	}
}