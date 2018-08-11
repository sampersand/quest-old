use std::borrow::Borrow;
use std::sync::{RwLock, atomic::{AtomicUsize, Ordering::Relaxed}};
use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter};
use std::mem;

pub enum IdType {
	Global,
	Local,
	Argument
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(usize);

lazy_static! {
	static ref STR_ID_MAPPINGS: RwLock<HashMap<&'static str, Id>> = RwLock::default();
	static ref ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
}

impl Id {
	pub(super) fn next() -> Id {
		Id(ID_COUNTER.fetch_add(1, Relaxed))
	}

	pub fn str_id(&self) -> Option<&'static str> {
		match STR_ID_MAPPINGS.read().unwrap().iter().find(|(k, v)| (*v == self)) {
			Some((str_id, _)) => Some(str_id),
			None => None
		}
	}

	pub fn from_nonstatic_str(str_id: &str) -> Id {
		assert!(!str_id.is_empty(), "cannot have emptpy string ids");
		if let Some(id) = STR_ID_MAPPINGS.read().unwrap().get(&str_id) {
			return *id;
		}

		let mut lock = STR_ID_MAPPINGS.write().unwrap();
		if let Some(id) = lock.get(str_id) {
			return *id;
		}

		let id = Id::next();
		let str_id = unsafe {
			let s = String::from(str_id);
			let str_id = mem::transmute(&s as &str);
			mem::forget(s);
			str_id
		};

		if let Some(old) = lock.insert(str_id, id) {
			panic!("old id encountered? `{:?}`", id);
		}
		id
	}

	pub fn classify(&self) -> Option<IdType> {
		let str_id = self.str_id()?;
		assert!(!str_id.is_empty(), "cannot have emptpy string ids");
		match str_id.chars().next().unwrap() {
			'@' => Some(IdType::Argument),
			'$' => Some(IdType::Global),
			_ => Some(IdType::Local)
		}
	}
}

impl Debug for Id {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self.str_id() {
			Some(id_str) if f.alternate() => f.debug_tuple("Id").field(&id_str).field(&self.0).finish(),
			Some(id_str) => f.debug_tuple("Id").field(&id_str).finish(),
			None => f.debug_tuple("Id").field(&self.0).finish()
		}
	}
}

impl Display for Id {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self.str_id() {
			Some(id_str) if f.alternate() => write!(f, "{}({})", id_str, self.0),
			Some(id_str) => write!(f, "{}", id_str),
			None => write!(f, "<{}>", self.0)
		}
	}
}

impl From<&'static str> for Id {
	fn from(str_id: &'static str) -> Id {
		if let Some(id) = STR_ID_MAPPINGS.read().unwrap().get(&str_id) {
			return *id;
		}

		let mut lock = STR_ID_MAPPINGS.write().unwrap();
		if let Some(id) = lock.get(str_id) {
			return *id;
		}

		let id = Id::next();
		if let Some(old) = lock.insert(str_id, id) {
			panic!("old id encountered? `{:?}`", id);
		}
		id
	}
}