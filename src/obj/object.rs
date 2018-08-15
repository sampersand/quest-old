use obj::{Id, AnyObject};

use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy)]
struct Defaults {
	// get: GetDefault,
	// has: HasDefault,
}

pub struct QObject<D: Send + Sync + ?Sized> {
	id: Id,
	attrs: HashMap<Id, AnyObject>,
	defaults: Defaults,
	data: D
}


impl<D: Send + Sync + ?Sized> QObject<D> {
	pub fn id(&self) -> Id {
		self.id
	}

	pub fn data(&self) -> &D {
		&self.data
	}
	pub fn data_mut(&mut self) -> &mut D {
		&mut self.data
	}
}

impl<D: Clone + Send + Sync + Sized> Clone for QObject<D> {
	fn clone(&self) -> Self {
		QObject {
			id: Id::next(), // if we clone ourself, we need a new Id
			attrs: self.attrs.clone(),
			data: self.data.clone(),
			defaults: self.defaults
		}
	}
}

impl<D: Send + Sync + Hash> Hash for QObject<D> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.id.hash(h)
	}
}

impl<D: Send + Sync + ?Sized> AsRef<D> for QObject<D> {
	#[inline]
	fn as_ref(&self) -> &D {
		self.data()
	}
}

impl<D: Eq + Send + Sync + ?Sized> Eq for QObject<D> {}
impl<D: PartialEq + Send + Sync + ?Sized> PartialEq for QObject<D> {
	fn eq(&self, other: &QObject<D>) -> bool {
		if cfg!(debug_assertions) && self.id == other.id {
			assert!(self.data == other.data, "datas dont match but ids do")
		}

		self.data == other.data
	}
}

impl<D: Debug + Send + Sync + ?Sized> Debug for QObject<D> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let mut f = f.debug_struct("QObject");
		f.field("id", &self.id);

		if !self.attrs.is_empty() {
			f.field("attrs", &self.attrs.keys());
		}

		f.field("data", &&self.data);
		f.finish()
	}
}

