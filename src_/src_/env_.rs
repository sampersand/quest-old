use std::sync::Arc;
use std::borrow::Borrow;
use shared::map::{SharedMap, MapGuard};
use sync_::{SyncMapReadGuard, SyncVec};
use std::ops::{Deref, DerefMut};
use builtins;
use std::collections::HashMap;
use std::sync::RwLock;
use obj_::{QObject__, Result_, Exception__, Id, classes_::{QMap, QVar}, IdType};
use parse::Token;
use std::fmt::{self, Debug, Formatter};

#[derive(Clone)]
pub struct Environment__<'a> {
	arguments: SharedMap<QObject__, QObject__>,
	locals: SharedMap<QObject__, QObject__>,
	globals: SharedMap<QObject__, QObject__>,
	specials: &'static HashMap<Id, fn(&Environment__) -> QObject__>,
	binding: Option<&'a Environment__<'a>>,
	pub tokens: SyncVec<&'static Token> //remove pub
}

lazy_static! {
	static ref SPECIALS: HashMap<Id, fn(&Environment__) -> QObject__> = {
		let mut h: HashMap<Id, fn(&Environment__) -> QObject__> = HashMap::new();

		fn get_env(env: &Environment__) -> QObject__ {
			unimplemented!("get_env")
		}
		fn get_globals(env: &Environment__) -> QObject__ {
			// QMap::new(env.globals.clone()).into()
			unimplemented!()
		}
		fn get_args(env: &Environment__) -> QObject__ {
			unimplemented!()
			// QMap::new(env.arguments.clone()).into()
		}
		fn get_locals(env: &Environment__) -> QObject__ {
			unimplemented!()
			// QMap::new(env.locals.clone()).into()
		}


		h.insert("$=".into(), get_env);
		h.insert("$_ENV".into(), get_env);

		h.insert("$*".into(), get_globals);
		h.insert("$_GLOBALS".into(), get_globals);

		h.insert("$_ARGS".into(), get_args);
		h.insert("@*".into(), get_args);

		h.insert("$_LOCALS".into(), get_locals);
		h
	};
}


impl<'a> Default for Environment__<'a> {
	fn default() -> Environment__<'a> {
		Environment__ {
			arguments: SharedMap::default(),
			locals: SharedMap::default(),
			globals: SharedMap::from(builtins::default_builtins().iter().map(|(&x, y)| (x.into(), y.clone())).collect::<HashMap<_, _>>()),
			tokens: SyncVec::from(Token::default_tokens().to_owned()),
			specials: &SPECIALS,
			binding: None
		}
	}
}

pub enum EnvReadGuard<'a> {
	Read(MapGuard<'a, QObject__, QObject__>),
	Dynamic(QObject__)
}

impl<'a> Deref for EnvReadGuard<'a> {
	type Target = QObject__;

	#[inline]
	fn deref(&self) -> &QObject__ {
		match self {
			EnvReadGuard::Read(thing) => thing.deref(),
			EnvReadGuard::Dynamic(ref value) => value
		}
	}
}

impl<'a> Environment__<'a> {
	pub fn set_arguments(&self, args: &[&QObject__]) {
		let mut arguments = self.arguments.write();

		for (i, arg) in args.iter().enumerate() {
			let id = QVar::from_nonstatic_str(&format!("@{}", i));
			arguments.insert(id.into(), (*arg).clone());
		}
	}

	pub fn bind(&mut self, env: &'a Environment__) {
		self.binding = Some(env);
	}

	pub fn clone_for_call(&self) -> Environment__ {
		Environment__ {
			globals: self.globals.clone(),
			..Environment__::default()
		}
	}

	pub fn try_get<Q: Borrow<QObject__>>(&self, index: &Q) -> Option<EnvReadGuard> {
		if let Some(value) = self.arguments.get(index) {
			return Some(EnvReadGuard::Read(value));
		}

		if let Some(value) = self.locals.get(index) {
			return Some(EnvReadGuard::Read(value));
		}

		if let Some(value) = self.globals.get(index) {
			return Some(EnvReadGuard::Read(value));
		}

		if let Some(dyn_func) = self.specials.get(&index.borrow().try_cast_var()?.into()) {
			return Some(EnvReadGuard::Dynamic(dyn_func(self)));
		} 

		if let Some(binding) = self.binding {
			binding.try_get(index)
		} else {
			None
		}
	}

	pub fn get(&self, index: &QObject__) -> Result_ {
		if let Some(obj) = self.try_get(index) {
			Ok(obj.clone().old())
		} else {
			Err(::obj_::Exception_::Old(Exception__::Missing(index.clone())))
		}
	}

	pub fn assign(&self, index: QObject__, value: QObject__) -> QObject__ {
	let which_place = match index.try_cast_var().map(|x| x.as_id()) {
			Some(id) => 
				match id.classify().expect("`QVar` without corresponding str?") {
					IdType::Global => unimplemented!(),//&self.globals,
					IdType::Local => &self.locals,
					IdType::Argument => &self.arguments
				},
			None => &self.locals
		};

		which_place.insert(index, value.clone()); // ignore the old value that was there
		value
	}

	pub fn has(&self, index: &QObject__) -> bool {
		self.arguments.contains_key(index)  ||
			self.locals.contains_key(index)  ||
			self.globals.contains_key(index) ||
			self.binding.map(|x| x.has(index)).unwrap_or(false)
	}
}

impl<'a> Debug for Environment__<'a> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Environment__")
			 .field("arguments", &*self.arguments.read())
			 .field("locals", &*self.locals.read())
			 .field("globals", &*self.globals.read())
			 .field("tokens", &self.tokens.read())
			 .field("binding", &self.binding)
			 .finish()
		} else {
			f.debug_struct("Environment__")
			 .field("arguments", &self.arguments.read().keys())
			 .field("locals", &self.locals.read().keys())
			 .field("globals", &self.globals.read().keys())
			 .field("binding", &self.binding)
			 .finish()
		}
	}
}