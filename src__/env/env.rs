use parse::{Stream, Token};
use std::borrow::Borrow;
use obj::{Id, Object, Error, AnyShared, AnyResult, SharedObject};
use obj::types::{self, Map, List, IntoObject};
use obj::Result;
use std::sync::Mutex;
use std::str::FromStr;
use env::Executable;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Hash)]
pub struct Environment {
	stack: SharedObject<List>,
	locals: SharedObject<Map>,
	parent: Option<Box<Environment>>
}

impl Default for Environment {
	fn default() -> Self {
		Environment {
			stack: Object::default(),
			locals: Object::default(),
			parent: Some(Box::new(Environment::global_env())),
		}
	}
}

lazy_static! {
	static ref VAR_ENV: AnyShared = Id::from("@env").into_object() as AnyShared;
	static ref VAR_LOCALS: AnyShared = Id::from("@locals").into_object() as AnyShared;
	static ref VAR_STACK: AnyShared = Id::from("@stack").into_object() as AnyShared;
}

impl Environment {
	pub fn new() -> Self {
		Environment::default()
	}
	pub fn global_env() -> Environment {
		Environment {
			stack: Object::default(),
			locals: super::default_globals().into_object(),
			parent: None
		}
	}


	pub fn new_stack(&self) -> Self {
		Environment {
			stack: Object::default(),
			..self.clone()
		}
	}

	pub fn new_binding(&self, args: &[AnyShared]) -> Self {
		Environment {
			parent: Some(Box::new(self.clone())),
			stack: Object::default(),
			locals: {
				let locals: SharedObject<Map> = Object::default();
				{
					let ref mut map = locals.write().data;
					for (i, arg) in args.iter().enumerate() {
						map.insert(Id::from_nonstatic_str(&format!("@{}", i + 1)).into_anyshared(), arg.clone());
					}
				}
				locals
			},
			.. Environment::empty()
		}
	}

	pub fn empty() -> Self {
		Environment {
			stack: Object::default(),
			locals: Object::default(),
			parent: None
		}
	}

	pub fn execute<I: Iterator<Item=Token>>(&self, mut iter: I) -> Result<()> {
		let mut peeker = (&mut iter as &mut dyn Iterator<Item=Token>).peekable();
		while let Some(token) = peeker.next() {
			match token.execute(self, &mut peeker) {
				Ok(()) => {},
				Err(Error::Return { env, obj }) =>
					if self.parent.is_none() || self.parent.as_ref().unwrap().as_ref() == &env.read().data {
						self.push(obj);
						break;
					} else {
						return Err(Error::Return { env, obj })
					},
				Err(err) => return Err(err)
			}
		}
		Ok(())
	}

	pub fn execute_stream(&self, stream: Stream) -> Result<()> {
		match self.execute(stream) {
			Err(Error::Return { obj, .. }) => {
				self.push(obj);
				Ok(())
			},
			other => other
		}
	}

	pub fn stack(&self) -> &SharedObject<List> {
		&self.stack
	}

	pub fn locals(&self) -> &SharedObject<Map> {
		&self.locals
	}

	pub fn parent(&self) -> Option<&Box<Environment>> {
		self.parent.as_ref()
	}
}

impl Environment {
	fn binding_iter(&self) -> impl Iterator<Item=&Environment> {
		struct BindingIter<'a>(Option<&'a Environment>);
		impl<'a> Iterator for BindingIter<'a> {
			type Item = &'a Environment;
			fn next(&mut self) -> Option<&'a Environment> {
				let ret = self.0?;
				self.0 = ret.parent.as_ref().map(Box::as_ref);
				return Some(ret);
			}
		}

		BindingIter(Some(self))
	}

	pub fn get_module_env(&self) -> &Environment {
		self.binding_iter().last().unwrap_or(self)
	}

	fn get_var(&self, var: &str) -> Option<AnyShared> {
		if var.chars().next()? != '$' {
			return None;
		}
		match &var[1..] {
			"env" => Some(self.clone().into_anyshared()),
			"stack" => Some(self.stack.clone() as AnyShared),
			"locals" => Some(self.locals.clone() as AnyShared),
			"bindings" => Some(self.binding_iter().map(|x| x.clone().into_anyshared()).collect::<Vec<_>>().into_anyshared()),
			_ => if let Ok(mut bind_pos) = isize::from_str(&var[1..]) {
				if bind_pos < 0 {
					bind_pos += self.binding_iter().count() as isize;
				}

				if bind_pos < 0 {
					Some(self.clone().into_anyshared())
				} else {
					Some(self.binding_iter()
						.nth(bind_pos as usize)
						.unwrap_or_else(|| self.get_module_env())
						.clone().into_anyshared())
				}
			} else {
				None
			}
		}
	}

	pub fn get(&self, key: &AnyShared) -> Option<AnyShared> {
		if let Some(val) = self.locals.read().data.get(key) {
			return Some(val.clone())
		}

		key.read().downcast_ref::<::obj::types::Var>()
			.and_then(|var| self.get_var(var.data.id_str()))
			.or_else(|| self.parent.as_ref().and_then(|p| p.get(key)))

		// if key == &*VAR_ENV {
		// 	return Some(self.env_object() as AnyShared);
		// }

		// if key == &*VAR_STACK {
		// 	return Some(self.stack.clone() as AnyShared);
		// }

		// if key == &*VAR_LOCALS {
		// 	return Some(self.locals.clone() as AnyShared);
		// }

		// if let Some(var) = key.read().downcast_ref::<::obj::types::Var>() {
		// 	let id_str = var.data.id_str();
		// 	if id_str.len() >= 2 {
		// 		match id_str.chars().next().unwrap() {
		// 			'@' => 
		// 				if let Ok(stack_pos) = usize::from_str(&id_str[1..]) {
		// 					if stack_pos == 0 {
		// 						return Some(self.parent.as_ref()?.env_object());
		// 					}
		// 					return Some(self.stack.read().data.get(stack_pos - 1).map(Clone::clone).unwrap_or_else(Object::null));
		// 				},
		// 			'$' => 
		// 				if let Ok(mut bind_pos) = isize::from_str(&id_str[1..]) {
		// 					if bind_pos < 0 {
		// 						bind_pos += self.binding_iter().count() as isize;
		// 					}

		// 					if bind_pos < 0 {
		// 						return Some(self.env_object());
		// 					}

		// 					return self.binding_iter().nth(bind_pos as usize).map(|x| x.env_object() as AnyShared);
		// 				},
		// 			_ => {}
		// 		}
		// 	}
		// }

		// if let Some(val) = self.parent.as_ref().map(|p| p.get(key)) {
		// 	return val
		// }

		// None
	}

	pub fn has(&self, key: &AnyShared) -> bool {
		self.locals.read().data.contains_key(key) || self.parent.as_ref().map(|p| p.has(key)).unwrap_or(false)
	}

	pub fn set(&self, key: AnyShared, val: AnyShared) -> AnyShared {
		if &key == &*VAR_LOCALS {
			unimplemented!("TODO: assign locals");
		}

		if &key == &*VAR_STACK {
			unimplemented!("TODO: assign stack");
		}

		if &key == &*VAR_ENV {
			unimplemented!("TODO: assign env");
		}

		self.locals.write().data.insert(key, val).unwrap_or_else(Object::null)
	}

	pub fn del(&self, key: &AnyShared) -> Option<AnyShared> {
		self.locals.write().data.remove(key)
	}

	pub fn push(&self, ele: AnyShared) {
		self.stack.write().data.push(ele)
	}

	pub fn pop(&self) -> Option<AnyShared> {
		self.stack.write().data.pop()
	}
}

impl Display for Environment {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "<env {} levels down>", self.binding_iter().count())
	}
}

impl Eq for Environment {}
impl PartialEq for Environment {
	fn eq(&self, other: &Environment) -> bool {
		self.locals == other.locals && self.parent == other.parent
	}
}


// impl IntoObject for Environment {
// 	type Type = Map;
// 	fn into_object(self) -> SharedObject<Map> {
// 		let mut map = Map::default();
// 		map.insert(VAR_LOCALS.clone(), self.locals.clone());
// 		map.insert(VAR_STACK.clone(), self.stack.clone());
// 		return map.into_object();
// 	}
// }