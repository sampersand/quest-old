use shared::SharedMap;

use std::mem;
use std::ops::Deref;
use std::borrow::Borrow;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::fmt::{self, Debug, Display, Formatter};

lazy_static! {
	static ref STR_ID_MAPPINGS: SharedMap<&'static str, Id> = SharedMap::default();
	static ref ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(usize);

impl Id {
	pub(super) fn next() -> Id {
		Id(ID_COUNTER.fetch_add(1, Relaxed))
	}

	pub fn try_as_str(&self) -> Option<&'static str> {
		STR_ID_MAPPINGS.read().iter().find(|(k, v)| (*v == self)).map(|(&s, _)| s)
	}

	pub fn from_nonstatic_str(str_id: &str) -> Id {
		assert!(!str_id.is_empty(), "cannot have emptpy string ids");
		if let Some(id) = STR_ID_MAPPINGS.read().get(&str_id) {
			return *id;
		}

		let mut lock = STR_ID_MAPPINGS.write();
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
}

impl From<&'static str> for Id {
	#[inline]
	fn from(inp: &'static str) -> Id {
		*(&inp).as_ref()
	}
}


impl AsRef<Id> for &'static str {
	fn as_ref(&self) -> &Id {
		// since we never remove Ids from the string mapping, its ok to make the lifetime static
		if let Some(id) = STR_ID_MAPPINGS.get(self) {
			return unsafe{ mem::transmute::<&Id, &'static Id>(id.deref()) };
		}

		let mut guard = STR_ID_MAPPINGS.write();
		if let Some(id) = guard.get(self) {
			return unsafe{ mem::transmute::<&Id, &'static Id>(id.deref()) };
		}

		assert_eq!(guard.insert(self, Id::next()), None);
		let id = guard.get(self).expect("id we just inserted doesn't exist?");
		return unsafe{ mem::transmute::<&Id, &'static Id>(id.deref()) };
	}
}

impl Borrow<Id> for &'static str {
	#[inline]
	fn borrow(&self) -> &Id {
		self.as_ref()
	}
}

impl Debug for Id {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if let Some(str_id) = self.try_as_str() {
			write!(f, "Id({:?}: {})", str_id, self.0) // to prevent the line-wrapping that occurs when `{:#?}` is used
		} else {
			write!(f, "Id({})", self.0)
		}
	}
}

impl Display for Id {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
 		if let Some(id_str) = self.try_as_str() {
			write!(f, "{}", id_str)
		} else {
			write!(f, "Id({})", self.0)
		}
	}
}
