use parse::Stream;
use std::borrow::Borrow;
use obj::{Id, Object, AnyShared, AnyResult, SharedObject};
use obj::types::{self, Map, List, IntoObject};
use std::sync::Mutex;

#[derive(Debug)]
pub struct Environment {
	self_obj: Mutex<Option<AnyShared>>,
	stack: SharedObject<List>,
	locals: SharedObject<Map>,
	globals: SharedObject<Map>,
}

impl Default for Environment {
	fn default() -> Self {
		Environment {
			self_obj: Mutex::new(None),
			stack: Object::default(),
			locals: Object::default(),
			globals: super::default_globals().into_object()
		}
	}
}

lazy_static! {
	static ref VAR_ENV: AnyShared = Id::from("@env").into_object() as AnyShared;
	static ref VAR_SELF: AnyShared = Id::from("@self").into_object() as AnyShared;
	static ref VAR_LOCALS: AnyShared = Id::from("locals").into_object() as AnyShared;
	static ref VAR_GLOBALS: AnyShared = Id::from("globals").into_object() as AnyShared;
	static ref VAR_TRUE: AnyShared = Id::from("true").into_object() as AnyShared;
	static ref VAR_FALSE: AnyShared = Id::from("false").into_object() as AnyShared;
}

impl Environment {
	pub fn execute_stream(&self, mut stream: Stream) {
		for obj in stream.iter(self) {
			self.push(obj);
		}
	}
}

impl Environment {
	fn env_object(&self) -> SharedObject<Map> {
		let mut map = Map::default();
		if let Some(ref self_obj) = &*self.self_obj.lock().unwrap() {
			map.insert(VAR_SELF.clone(), self_obj.clone());
		}
		map.insert(VAR_LOCALS.clone(), self.locals.clone());
		map.insert(VAR_GLOBALS.clone(), self.globals.clone());
		return map.into_object();
	}

	pub fn get(&self, key: &AnyShared) -> Option<AnyShared> {
		if let Some(val) = self.locals.read().data.get(key) {
			return Some(val.clone())
		}

		if let Some(val) = self.globals.read().data.get(key) {
			return Some(val.clone())
		}

		if key == &*VAR_ENV {
			return Some(self.env_object() as AnyShared);
		}

		if key == &*VAR_SELF {
			return Some(self.self_obj.lock().unwrap().as_ref().cloned().unwrap_or_else(Object::null));
		}

		None
	}

	pub fn has(&self, key: &AnyShared) -> bool {
		self.locals.read().data.contains_key(key) || self.globals.read().data.contains_key(key)
	}

	pub fn set(&self, key: AnyShared, val: AnyShared) -> AnyShared {
		if &key == &*VAR_SELF {
			let mut lock = self.self_obj.lock().unwrap();
			let old = lock.take();
			*lock = Some(val);
			return old.unwrap_or_else(Object::null);
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
