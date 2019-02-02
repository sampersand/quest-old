mod types;
mod map;

use self::types::Type;
use self::map::ObjectMap;

use std::sync::{Arc, RwLock, Weak};
use std::any::Any;
use crate::shared::Shared;
use crate::map::Map;
use crate::err::{Error, Result};
use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Formatter};

pub struct Object<T: ?Sized + Send + Sync>(Arc<Inner<T>>);
pub type AnyObject = Object<dyn Any + Send + Sync>;

struct InternalOps {
	hash: fn(&dyn Any, &mut Hasher),
	debug: fn(&dyn Any, &mut Formatter) -> fmt::Result
}

#[derive(Debug)]
struct ObjectInfo {
	id: usize,
	env: Shared<dyn Map>,
	parent: Option<AnyObject>,
}

struct Inner<T: ?Sized + Send + Sync> {
	map: Shared<ObjectMap>,
	info: ObjectInfo,
	ops: InternalOps,
	weakref: Weak<Inner<dyn Any + Send + Sync>>, // is this required?
	data: RwLock<T>,
}

impl<T: Type + Sized> Object<T> {
	#[cfg_attr(feature = "ignore-unused", allow(unused))]
	pub fn new(data: T) -> Object<T> {
		Object::_new(data, None)
	}

	#[cfg_attr(feature = "ignore-unused", allow(unused))]
	pub fn new_child(data: T, parent: AnyObject) -> Object<T> {
		Object::_new(data, Some(parent))
	}

	fn _new(data: T, parent: Option<AnyObject>) -> Object<T> {
		use std::sync::atomic::{AtomicUsize, Ordering};

		lazy_static::lazy_static! {
			static ref ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
		}

		let inner = Arc::new(Inner {
			map: Shared::new(ObjectMap::from_type::<T>()),
			info: ObjectInfo {
				id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
				env: crate::env::current(),
				parent: parent,
			},
			ops: InternalOps {
				hash: |obj, mut hasher| Any::downcast_ref::<T>(obj).expect("bad obj passed to `hasher`").hash(&mut hasher),
				debug: |obj, f| Debug::fmt(Any::downcast_ref::<T>(obj).expect("bad obj passed to `fmt`"), f)
			},
			weakref: unsafe { std::mem::uninitialized() },
			data: RwLock::new(data)
		});

		let mut obj = Object(inner);
		unsafe { 
			std::ptr::write(
				&mut Arc::get_mut(&mut obj.0).unwrap().weakref as *mut Weak<_>,
				Arc::downgrade(&obj.0) as _
			);
		}
		obj
	}
}

impl<T: Send + Sync + ?Sized> Object<T> {
	#[cfg_attr(feature = "ignore-unused", allow(unused))]
	pub fn parent(&self) -> Option<AnyObject> {
		self.0.info.parent.clone()
	}

	#[cfg_attr(feature = "ignore-unused", allow(unused))]
	pub fn id(&self) -> usize {
		self.0.info.id
	}

	#[cfg_attr(feature = "ignore-unused", allow(unused))]
	pub fn env(&self) -> Shared<dyn Map> {
		self.0.info.env.clone()
	}

	#[cfg_attr(feature = "ignore-unused", allow(unused))]
	pub fn data(&self) -> &RwLock<T> {
		&self.0.data
	}
}

impl<T: Send + Sync + Sized + 'static> Object<T> {
	pub fn call_attr(&self, attr: &'static str, args: &[&AnyObject]) -> Result<AnyObject> {
		self.as_any().call_attr(attr, args)
	}

	pub fn call(&self, attr: &AnyObject, args: &[&AnyObject]) -> Result<AnyObject> {
		self.as_any().call(attr, args)
	}
}

impl AnyObject {
	pub fn call_attr(&self, attr: &'static str, args: &[&AnyObject]) -> Result<AnyObject> {
		self.call(&Object::new_variable(attr).as_any(), args)
	}

	pub fn call(&self, attr: &AnyObject, args: &[&AnyObject]) -> Result<AnyObject> {
		let val = self.0.map.read().expect("cant read obj map").get(attr)
			.ok_or_else(|| Error::AttrMissing { obj: self.clone(), attr: attr.clone()})?;

		match val.downcast::<types::RustFn>() {
			Some(rustfn) => rustfn.data().read().expect("err when calling rustfn").call(self, args),
			None => {
				let mut self_args = Vec::with_capacity(args.len() + 1);
				self_args.push(self);
				self_args.extend(args);
				val.call_attr("()", self_args.as_ref())
			}
		}

	}
}

impl<T: 'static + Send + Sync + Sized> Object<T> {
	pub fn as_any(&self) -> AnyObject {
		Object(self.0.clone() as _)
	}
}

impl AnyObject {
	pub fn downcast_or_err<T: Send + Sync + 'static>(&self) -> Result<Object<T>> {
		self.downcast::<T>().ok_or_else(|| Error::CastError {
			obj: self.clone(),
			into: type_name::get::<T>()
		})
	}

	pub fn downcast<T: Send + Sync + 'static>(&self) -> Option<Object<T>> {
		if self.0.data.read().unwrap().is::<T>() {
			Some(Object(unsafe {
				Arc::from_raw(Arc::into_raw(self.0.clone()) as *const Inner<T>)
			}))
		} else {
			None
		}
	}
}

impl Debug for InternalOps {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		use crate::util::PtrFormatter;
		f.debug_struct("ObjectInfo")
		 .field("hash", &PtrFormatter(self.hash as usize))
		 .field("debug", &PtrFormatter(self.debug as usize))
		 .finish()
	}
}

impl<T: Send + Sync + ?Sized> Clone for Object<T> {
	fn clone(&self) -> Object<T> {
		Object(self.0.clone())
	}
}

impl Eq for AnyObject {}
impl PartialEq for AnyObject {
	fn eq(&self, rhs: &AnyObject) -> bool {
		use self::types::{Variable, Boolean};

		if let (Some(lhs), Some(rhs)) = (self.downcast::<Variable>(), rhs.downcast::<Variable>()) {
			let lhs = lhs.data().read().expect("TODO: msg");
			let rhs = rhs.data().read().expect("TODO: msg");
			*lhs == *rhs
		} else {
			self.call_attr("==", &[rhs])
				.ok()
				.and_then(|x| x.downcast::<Boolean>())
				.map(|obj| obj.data().read().expect("TODO: msg").is_true())
				.unwrap_or(false)
		}
	}
}

impl<T: Send + Sync + Sized + 'static> Eq for Object<T> {}
impl<T: Send + Sync + Sized + 'static> PartialEq for Object<T> {
	fn eq(&self, rhs: &Object<T>) -> bool {
		// this is a really awkward way to do it, but whatever?
		// especially if T is hashable on its own, this might lead to weird situations
		self.as_any() == rhs.as_any()
	}
}

impl Debug for AnyObject {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		struct DataFmtr<'a>(&'a AnyObject);

		impl Debug for DataFmtr<'_> {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				let data = self.0.data().read().expect("read err in DataFmtr::fmt");
				((self.0).0.ops.debug)(&*data, f)
			}
		}

		if f.alternate() {
			f.debug_struct("Object")
			 .field("map", &self.0.map)
			 .field("info", &self.0.info)
			 .field("data", &DataFmtr(self))
			 .finish()
		} else {
			write!(f, "Object({:?})", DataFmtr(self))
		}
	}
}

impl<T: Send + Sync + Sized + 'static> Debug for Object<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		// this is a really awkward way to do it, but whatever?
		// especially if T is hashable on its own, this might lead to weird situations
		Debug::fmt(&self.as_any(), f)
	}
}


impl Hash for AnyObject {
	fn hash<H: Hasher>(&self, h: &mut H) {
		let data = self.data().read().expect("read error in AnyObject::hash");
		(self.0.ops.hash)(&*data, h);
	}
}

impl<T: Send + Sync + Sized + 'static> Hash for Object<T> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		// this is a really awkward way to do it, but whatever?
		// especially if T is hashable on its own, this might lead to weird situations
		self.as_any().hash(h)
	}
}


impl<T: std::marker::Unsize<U> + Send + Sync + ?Sized, U: Send + Sync + ?Sized> std::ops::CoerceUnsized<Object<U>> for Object<T> {}


#[cfg(test)]
mod tests {
	use super::*;
	#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
	struct MyType(i32);

	impl Type for MyType {
		fn get_type_map() -> Shared<dyn Map> {
			unimplemented!("TODO: type for MyType")
		}
	}

	// #[allow(unused)]
	fn _check_is_sized() {
		fn _is_sized<T: Sized>(){}
		_is_sized::<AnyObject>()
	}

	#[test]
	fn new() {
		let obj: Object<MyType> = Object::new(MyType(123));
		assert_eq!(obj.parent(), None);
		assert_eq!(*obj.data().read().unwrap(), MyType(123));

		let obj = obj.as_any();
		let obj2: Object<MyType> = Object::new_child(MyType(456), obj.clone());

		assert_eq!(obj2.parent(), Some(obj));
		assert_eq!(*obj2.data().read().unwrap(), MyType(456));
	}

	#[test]
	fn hashing() {
		fn hash<T: Hash>(t: &T) -> u64 {
			let mut s = std::collections::hash_map::DefaultHasher::new();
			t.hash(&mut s);
			s.finish()
		}

		let myt = MyType(-123_456);
		let ref obj = Object::new(myt);

		assert_eq!(hash(obj), hash(obj));
		assert_eq!(hash(obj), hash(&obj.as_any()));
		assert_eq!(hash(&obj.as_any()), hash(&obj.as_any()));
	}

	#[test]
	fn equality() {
		let ref obj1 = Object::new(MyType(234));
		let ref obj2 = Object::new(MyType(567));

		assert_eq!(obj1, &obj1.clone());
		assert_ne!(obj1, obj2);

		assert_eq!(&obj1.as_any(), &obj1.as_any().clone());
		assert_ne!(&obj1.as_any(), &obj2.as_any());
	}
}






