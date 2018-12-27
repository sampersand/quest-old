use obj::{Id, AnyObject, SharedObject, classes::{Class, GetDefault, HasDefault}, Result};
use env::Environment;
use std::borrow::Borrow;

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


impl AnyObject {
	pub fn get_attr<Q: Borrow<Id>>(&self, attr: Q) -> Option<AnyObject> {
		let attr = attr.borrow();
		if let Some(obj) = self.read().attrs.get(attr) {
			Some(obj.clone())
		} else {
			(self.read().defaults.get)(self, attr)
		}
	}


	pub fn has_attr<Q: Borrow<Id>>(&self, attr: Q) -> bool {
		let attr = attr.borrow();
		self.read().attrs.contains_key(attr) || (self.read().defaults.has)(attr)
	}

	pub fn set_attr<Q: Into<Id>>(&self, attr: Q, obj: AnyObject) -> Option<AnyObject> {
		let ref mut attrs = self.write().attrs;
		let attr = attr.into();
		let prev = attrs.remove_entry(&attr).map(|(_, v)| v);
		assert!(attrs.insert(attr, obj).is_none());
		prev
	}

	pub fn del_attr<Q: Borrow<Id>>(&self, attr: Q) -> Option<AnyObject> {
		self.write().attrs.remove_entry(attr.borrow()).map(|(_, obj)| obj)
	}

	pub fn call_attr<Q: Borrow<Id>>(&self, attr: Q, args: &[&AnyObject], env: &Environment) -> Result<AnyObject> {
		if let Some(attr) = self.get_attr(attr.borrow()) {
			// this is playing with fire here; we need a better way to determine the type at runtime
			unsafe {
				::obj::classes::QBoundFn::from_raw(attr.into_raw())
			}.call(args, env)
			// attr.call_attr("()", args, env);
			// attr.into_raw();
			// Ok(unimplemented!())
		} else {
			panic!("Attribute `{}` doesn't exist for {:#?}", attr.borrow(), self)
		}
		// self.get_attr(attr)?.ca
		// if let Some(qboundfn) = self.attrs.get(id.clone(), self) {
		// 	if let Classes::BoundFn(boundfn) = qboundfn.data().deref() {
		// 		boundfn.call(args, env)
		// 	} else {
		// 		panic!("BoundFn is needed to call attr")
		// 	}
		// } else {
		// 	warn!("Missing attribute {} for {:?}", id, self);
		// 	Ok(().into())
		// }
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





