pub mod builtins;

use crate::{Shared, Object, Result, parse::Parser};
use crate::collections::{Collection, Mapping, Listing};
use std::fmt::{self, Display, Formatter};
use std::{mem, sync::RwLock};
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Environment {
	id: usize,
	parent: Option<Shared<Environment>>,
	parser: Shared<Parser>,
	map: Shared<dyn Mapping>,
	pub(crate) stack: Shared<dyn Listing>
}

impl Eq for Environment {}
impl PartialEq for Environment {
	fn eq(&self, other: &Environment) -> bool {
		self.id == other.id
	}
}
impl Environment {
	fn next_id() -> usize {
		use std::sync::atomic::{AtomicUsize, Ordering};
		lazy_static! {
			static ref ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
		}

		ID_COUNTER.fetch_add(1, Ordering::Relaxed)
	}

	fn empty() -> Environment {
		Environment {
			id: Environment::next_id(),
			parent: None,
			parser: Shared::new(Parser::default()),
			map: Shared::new(crate::collections::Map::empty()),
			stack: Shared::new(crate::collections::List::empty())
		}
	}

	pub fn new(parser: Shared<Parser>, parent: Option<Shared<Environment>>, map: Option<Shared<dyn Mapping>>, stack: Option<Shared<dyn Listing>>) -> Shared<Environment> {
		Shared::new(Environment {
			parser, parent,
			map: map.unwrap_or_else(|| Shared::new(crate::collections::Map::empty())),
			stack: stack.unwrap_or_else(|| Shared::new(crate::collections::List::empty())),
			id: Environment::next_id()
		})
	}


	// im not sure how i want initialization to work, that's why this is underscored
	pub fn _new_default_with_stream_and_parent(parser: Shared<Parser>, parent: Option<Shared<Environment>>) -> Shared<Environment> {
		Shared::new(Environment {
			parser, parent,
			map: Shared::new(crate::collections::ParentalMap::new_default(|| builtins::BUILTINS_MAP.clone())),
			..Environment::empty()
		})
	}

	pub fn _new_default_with_stream_using_parent_stack(parser: Shared<Parser>, parent: Option<Shared<Environment>>) -> Shared<Environment> {
		Shared::new(Environment {
			stack: parent.as_ref().map(|p| p.read().stack.clone()).unwrap_or_else(|| Environment::empty().stack.clone()),
			parser, parent,
			map: Shared::new(crate::collections::ParentalMap::new_default(|| builtins::BUILTINS_MAP.clone())),
			..Environment::empty()
		})
	}


	pub fn execute(env: Shared<Environment>) -> Result<Shared<Environment>> {
		trace!(target: "execute", "Starting to execute");
		let mut parser = env.read().parser.clone();
		let old_env = Environment::set_current(env);

		loop {
			match Parser::next_unevaluated_object(&parser).transpose() {
				Err(crate::Error::NothingToReturn) => continue,
				Err(err) => { Environment::set_current(old_env); return Err(err) },
				Ok(Some(object)) => match object.evaluate(&parser) {
					Err(crate::Error::NothingToReturn) => continue,
					Err(crate::Error::Return { env, obj }) => {
						if env == Environment::current() {
							if let Some(object) = obj {
								trace!(target: "execute", "Env received next object from return statement: {:?}", object);
								Environment::current().read().stack.write().push(object);
							} else {
								continue
							}
						} else {
							Environment::set_current(old_env);
							return Err(crate::Error::Return { env, obj })
						}
					},
					Err(err) => {Environment::set_current(old_env); return Err(err) },
					Ok(object) => {
						trace!(target: "execute", "Env received next object: {:?}", object);
						Environment::current().read().stack.write().push(object);
					}
				},
				Ok(None) => break
			}
		}

		Ok(Environment::set_current(old_env))
	}
}

/** CURRENT for env **/
lazy_static! {
	static ref CURRENT: RwLock<Shared<Environment>> = RwLock::new(Environment::_new_default_with_stream_and_parent(Shared::new(Parser::default()), None));
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

impl_typed_object!(Shared<Environment>, variant Env, new_env, downcast_env, is_env);
impl_quest_conversion!("@env" (as_env_obj is_env) (into_env downcast_env) -> Shared<Environment>);
impl_type! { for Shared<Environment>, downcast_fn=downcast_env;
	fn "@env" (this) { this.into_object() }
	// todo: stuff here?
}

impl Collection for Environment {
	fn len(&self) -> usize {
		self.map.len()
	}

	fn is_empty(&self) -> bool {
		self.map.is_empty()
	}
}

impl Environment {
	fn get_special(&self, key: &str) -> Option<Object> {
		use std::str::FromStr;

		let sigil = key.chars().next().unwrap();
		debug_assert!(sigil == '@' || sigil == '$');
		let key: &str = &key[1..];
		if sigil == '@' {

			if let Ok(mut nth) = isize::from_str(key) {
				let stack = self.stack.read()._to_vec();
				if nth < 0 {
					if (-nth as usize) < stack.len() {
						nth += stack.len() as isize;
					} else {
						return None;
					}
				}
				return stack.get(nth as usize).cloned();
			}
		} else if sigil == '$' {
			use crate::object::IntoObject;
			if key == "stack" {
				return Some(self.stack.clone().into_object())
			} else if key == "locals" {
				return Some(self.map.clone().into_object())
			} else if let Ok(mut nth) = isize::from_str(key) {
				let mut env_stack = vec![/* and self here in the future */];
				let mut p = self.parent.clone();
				while let Some(parent) = p {
					p = parent.read().parent.clone();
					env_stack.push(parent);
				}

				if nth < 0 {
					if (-nth as usize) < env_stack.len() {
						nth += env_stack.len() as isize;
					} else {
						return None;
					}
				} else {
					nth = nth - 1; // temporary, until i get a `this` working
				}
				use crate::object::IntoObject;
				return env_stack.get(nth as usize).cloned().map(|x| x.into_object());
			}
		}

		None
	}
}

impl Mapping for Environment {
	fn duplicate(&self) -> Shared<dyn Mapping> {
		unimplemented!("duplicate")
	}

	fn get(&self, key: &Object) -> Option<Object> {
		// hack to allow for special $-vars
		if let Some(var) = key.downcast_var() {
			match var.as_ref().chars().next() {
				Some('$') | Some('@') => if let Some(special) = self.get_special(var.as_ref()) {
					return Some(special);
				},
				_ => {}
			}
		}

		self.map.get(key).or_else(|| self.parent.as_ref().and_then(|parent| parent.get(key)))
	}

	fn set(&mut self, key: Object, val: Object) -> Option<Object> {
		self.map.set(key, val)
	}

	fn del(&mut self, key: &Object) -> Option<Object> {
		self.map.del(key)
	}

	fn has(&self, key: &Object) -> bool {
		// todo: get special for has
		self.map.has(key) || self.parent.as_ref().map(|parent| parent.has(key)).unwrap_or(false)
	}
}