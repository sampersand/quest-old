use obj::{Id, AnyObject, SharedObject, classes::{Class, GetDefault, HasDefault}};

use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy)]
struct Defaults { 
	get: GetDefault,
	has: HasDefault,
	fmt: fn(&dyn Any, f: &mut Formatter) -> fmt::Result,
}

pub struct QObject<D: Send + Sync + ?Sized> {
	id: Id,
	attrs: HashMap<Id, AnyObject>,
	defaults: Defaults,
	data: D
}

impl<D: Debug + Send + Sync> QObject<D> where SharedObject<D>: Class {
	fn new(data: D) -> QObject<D> {
		QObject {
			id: Id::next(),
			attrs: HashMap::default(),
			defaults: Defaults {
				get: SharedObject::<D>::GET_DEFAULTS,
				has: SharedObject::<D>::HAS_DEFAULTS,
				fmt: |x, f| if f.alternate() {
					write!(f, "{:#?}", x.downcast_ref::<D>().expect("invalid argument passed to formatter"))
				} else {
					write!(f, "{:?}", x.downcast_ref::<D>().expect("invalid argument passed to formatter"))
				}
			},
			data
		}
	}
}

impl AnyObject {
	pub fn downcast<T: Send + Sync>(self) -> ::std::result::Result<SharedObject<T>, AnyObject> where SharedObject<T>: Class {
		use std::any::Any;

		let is_t = {
			let qobject = self.read();
			Any::is::<T>(qobject.data() as &Any)
		};
		if is_t {
			unsafe {
				Ok(SharedObject::<T>::from_raw(self.into_raw()))
			}
		} else {
			Err(self)
		}
	}
}


impl<D: Send + Sync + ?Sized> Deref for QObject<D> {
	type Target = D;

	#[inline]
	fn deref(&self) -> &D {
		&self.data
	}
}

impl<D: Send + Sync + ?Sized> DerefMut for QObject<D> {
	#[inline]
	fn deref_mut(&mut self) -> &mut D {
		&mut self.data
	}
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

impl<D: Debug + Send + Sync> From<D> for SharedObject<D> where SharedObject<D>: Class {
	#[inline]
	fn from(data: D) -> SharedObject<D> {
		QObject::new(data).into()
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
default impl<D: Debug + Send + Sync + ?Sized> Debug for QObject<D> {
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

impl Debug for QObject<dyn Any + Send + Sync> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let mut f = f.debug_struct("QObject");
		f.field("id", &self.id);

		if !self.attrs.is_empty() {
			f.field("attrs", &self.attrs.keys());
		}

		struct AnyFormatter<'a>(&'a dyn Any, fn(&'a dyn Any, f: &mut Formatter) -> fmt::Result);
		impl<'a> Debug for AnyFormatter<'a> {
			#[inline]
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				(self.1)(self.0, f)
			}
		}

		f.field("data", &AnyFormatter(&self.data, self.defaults.fmt));
		f.finish()
	}
}





