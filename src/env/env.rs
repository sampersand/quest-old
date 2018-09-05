use parse::{Stream, Token};
use std::borrow::Borrow;
use obj::{Id, Object, Error, AnyShared, AnyResult, SharedObject};
use obj::types::{self, Map, List, IntoObject};
use obj::Result;
use std::sync::Mutex;
use std::str::FromStr;
use env::Executable;

#[derive(Debug, Clone)]
pub struct Environment {
	stack: SharedObject<List>,
	locals: SharedObject<Map>,
	binding: Option<Box<Environment>>
}

impl Default for Environment {
	fn default() -> Self {
		Environment {
			stack: Object::default(),
			locals: super::default_globals().into_object(),
			binding: None
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


	pub fn new_stack(&self) -> Self {
		Environment {
			stack: Object::default(),
			..self.clone()
		}
	}

	pub fn new_binding(&self, args: &[AnyShared]) -> Self {
		Environment {
			binding: Some(Box::new(self.clone())),
			stack: args.into_object(),
			.. Environment::empty()
		}
	}

	pub fn empty() -> Self {
		Environment {
			stack: Object::default(),
			locals: Object::default(),
			binding: None
		}
	}

	pub fn execute<I: Iterator<Item=Token>>(&self, mut iter: I) -> Result<()> {
		let mut peeker = (&mut iter as &mut dyn Iterator<Item=Token>).peekable();
		while let Some(token) = peeker.next() {
			match token.execute(self, &mut peeker) {
				Ok(()) => {},
				Err(Error::Return { levels: 0, obj }) => {
					self.push(obj);
					break;
				},
				Err(Error::Return { levels, obj }) => return Err(Error::Return { levels: levels - 1, obj }),
				Err(err) => return Err(err)
			}
		}
		Ok(())
	}

	pub fn execute_stream(&self, stream: Stream) -> Result<()> {
		match self.execute(stream) {
			Err(Error::Return { levels: _, obj }) => {
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
}

impl IntoObject for Environment {
	type Type = Map;
	fn into_object(self) -> SharedObject<Map> {
		let mut map = Map::default();
		map.insert(VAR_LOCALS.clone(), self.locals.clone());
		map.insert(VAR_STACK.clone(), self.stack.clone());
		return map.into_object();
	}
}

impl Environment {
	fn binding_iter(&self) -> impl Iterator<Item=&Environment> {
		struct BindingIter<'a>(Option<&'a Environment>);
		impl<'a> Iterator for BindingIter<'a> {
			type Item = &'a Environment;
			fn next(&mut self) -> Option<&'a Environment> {
				let binding = self.0?.binding.as_ref();
				self.0 = binding.map(Box::as_ref);
				return self.0;
			}
		}

		BindingIter(Some(self))
	}

	fn get_var(&self, var: &str) -> Option<AnyShared> {
		if var.chars().next()? != '@' {
			return None;
		}

		match &var[1..] {
			"env" => Some(self.clone().into_anyshared()),
			"stack" => Some(self.stack.clone() as AnyShared),
			"locals" => Some(self.locals.clone() as AnyShared),
			"binding" => Some(self.binding_iter().map(|x| x.clone().into_anyshared()).collect::<Vec<_>>().into_anyshared()),
			// x if var[0] == '@' => @locals.0,
			_ => None
		}
	}

	pub fn get(&self, key: &AnyShared) -> Option<AnyShared> {
		if let Some(val) = self.locals.read().data.get(key) {
			return Some(val.clone())
		}

		key.read().downcast_ref::<::obj::types::Var>()
			.and_then(|var| self.get_var(var.data.id_str()))
			.or_else(|| self.binding.as_ref().and_then(|p| p.get(key)))

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
		// 						return Some(self.binding.as_ref()?.env_object());
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

		// if let Some(val) = self.binding.as_ref().map(|p| p.get(key)) {
		// 	return val
		// }

		// None
	}

	pub fn has(&self, key: &AnyShared) -> bool {
		self.locals.read().data.contains_key(key) || self.binding.as_ref().map(|p| p.has(key)).unwrap_or(false)
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
