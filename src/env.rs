use std::borrow::Borrow;
use sync::{SyncMap, SyncMapReadGuard, SyncVec};
use std::ops::{Deref, DerefMut};
use builtins;
use std::collections::HashMap;
use std::sync::RwLock;
use obj::{QObject, Id, classes::{QMap, QVar}, IdType};
use parse::Token;
use std::fmt::{self, Debug, Formatter};

#[derive(Clone)]
pub struct Environment<'a> {
	arguments: SyncMap<QObject, QObject>,
	locals: SyncMap<QObject, QObject>,
	globals: SyncMap<QObject, QObject>,
	specials: &'static HashMap<Id, fn(&Environment) -> QObject>,
	binding: Option<&'a Environment<'a>>,
	pub tokens: SyncVec<&'static Token> //remove pub
}

lazy_static! {
	static ref SPECIALS: HashMap<Id, fn(&Environment) -> QObject> = {
		let mut h: HashMap<Id, fn(&Environment) -> QObject> = HashMap::new();

		fn get_env(env: &Environment) -> QObject {
			unimplemented!("get_env")
		}
		fn get_globals(env: &Environment) -> QObject {
			unimplemented!("get globals")
		}
		fn get_args(env: &Environment) -> QObject {
			unimplemented!("get arguments")
		}
		fn get_locals(env: &Environment) -> QObject {
			unimplemented!("get locals")
		}


		h.insert("$=".into(), get_env);
		h.insert("$_ENV".into(), get_env);

		h.insert("$$".into(), get_globals);
		h.insert("$_GLOBALS".into(), get_globals);

		h.insert("$@".into(), get_args);
		h.insert("$_ARGS".into(), get_args);
		h.insert("@@".into(), get_args);

		h.insert("$.".into(), get_locals);
		h.insert("$_LOCALS".into(), get_locals);
		h
	};
}


impl<'a> Default for Environment<'a> {
	fn default() -> Environment<'a> {
		Environment {
			arguments: SyncMap::new(),
			locals: SyncMap::new(),
			globals: SyncMap::from(builtins::default_builtins().iter().map(|(&x, y)| (x.into(), y.clone())).collect::<HashMap<_, _>>()),
			tokens: SyncVec::from(Token::default_tokens().to_owned()),
			specials: &SPECIALS,
			binding: None
		}
	}
}

pub enum ReadGuard<'b: 'c, 'c, Q: Borrow<QObject> + 'c> {
	Read(SyncMapReadGuard<'b, 'c, QObject, QObject, Q>),
	Dynamic(QObject)
}

impl<'b: 'c, 'c, Q: Borrow<QObject>> Deref for ReadGuard<'b, 'c, Q> {
	type Target = QObject;

	#[inline]
	fn deref(&self) -> &QObject {
		match self {
			ReadGuard::Read(thing) => thing.deref(),
			ReadGuard::Dynamic(ref value) => value
		}
	}
}

impl<'a> Environment<'a> {
	pub fn set_arguments(&self, args: &[&QObject]) {
		let mut arguments = HashMap::<QObject, QObject>::new();

		for (i, arg) in args.iter().enumerate() {
			let id = QVar::from_nonstatic_str(&format!("@{}", i));
			arguments.insert(id.into(), (*arg).clone());
		}

		arguments.insert("@*".into(), args.len().into());

		*self.arguments.lock().deref_mut() = arguments;
	}

	pub fn clone_for_call<'b>(&'b self, args: &[&QObject]) -> Environment<'b> {
		let mut arguments = HashMap::<QObject, QObject>::new();

		for (i, arg) in args.iter().enumerate() {
			let id = QVar::from_nonstatic_str(&format!("@{}", i));
			arguments.insert(id.into(), (*arg).clone());
		}

		arguments.insert("@*".into(), args.len().into());

		Environment {
			arguments: arguments.into(),
			binding: Some(self),
			..Environment::default()
		}
	}

	pub fn try_get<'b: 'c, 'c, Q: Borrow<QObject>>(&'b self, index: &'c Q) -> Option<ReadGuard<'b, 'c, Q>> {
		if let Some(value) = self.arguments.get(index) {
			return Some(ReadGuard::Read(value));
		}

		if let Some(value) = self.locals.get(index) {
			return Some(ReadGuard::Read(value));
		}

		if let Some(value) = self.globals.get(index) {
			return Some(ReadGuard::Read(value));
		}

		let id: Id = index.borrow().try_cast_var()?.into();

		if let Some(dyn_func) = self.specials.get(&id) {
			Some(ReadGuard::Dynamic(dyn_func(self)))
		} else if let Some(binding) = self.binding {
			binding.try_get(index)
		} else {
			None
		}
	}

	pub fn get(&self, index: &QObject) -> QObject {
		if let Some(obj) = self.try_get(index) {
			obj.clone()
		} else {
			info!("The index ({}) doesn't exist in the current environment ({:?})", index, self);
			().into()
		}
	}

	pub fn set(&self, index: QObject, value: QObject) -> QObject {
	let which_place = match index.try_cast_var().map(|x| x.as_id()) {
			Some(id) => 
				match id.classify().expect("`QVar` without corresponding str?") {
					IdType::Global => &self.globals,
					IdType::Local => &self.locals,
					IdType::Argument => &self.arguments
				},
			None => &self.locals
		};

		which_place.set(index, value.clone()); // ignore the old value that was there
		value
	}

	pub fn has(&self, index: &QObject) -> bool {
		self.arguments.has_key(index) || self.locals.has_key(index) || self.globals.has_key(index)
	}
}

impl<'a> Debug for Environment<'a> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Environment")
			 .field("locals", &*self.locals.read())
			 .field("globals", &*self.globals.read())
			 .field("tokens", &self.tokens.read())
			 .finish()
		} else {
			f.debug_struct("Environment")
			 .field("locals", &self.locals.read().keys())
			 .field("globals", &self.globals.read().keys())
			 .finish()
		}
	}
}