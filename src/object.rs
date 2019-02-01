mod types;
mod map;

use self::types::Type;
use self::map::ObjectMap;

use std::sync::{Arc, Weak};
use std::any::Any;
use crate::shared::Shared;
use crate::map::Map;
use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Formatter};

#[derive(Debug)]
pub struct Object<T: ?Sized + Send + Sync>(Arc<Inner<T>>);
pub type AnyObject = Object<dyn Any + Send + Sync>;

struct ObjectInfo {
	id: usize,
	env: Shared<dyn Map>,
	parent: Option<AnyObject>,
	hash: fn(&dyn Any, &mut Hasher)
}

#[derive(Debug)]
struct Inner<T: ?Sized + Send + Sync> {
	map: Shared<ObjectMap>,
	info: ObjectInfo,
	weakref: Weak<Inner<dyn Any + Send + Sync>>,
	data: T
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
				hash: (|obj, mut hasher| Any::downcast_ref::<T>(obj).expect("bad obj passed to hasher").hash(&mut hasher))
			},
			weakref: unsafe { ::std::mem::uninitialized() },
			data: data
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
	pub fn data(&self) -> &T {
		&self.0.data
	}
}

impl<T: 'static + Send + Sync + Sized> Object<T> {
	#[cfg_attr(feature = "ignore-unused", allow(unused))]
	fn as_any(&self) -> AnyObject {
		Object(self.0.clone() as _)
	}
}

impl Debug for ObjectInfo {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		struct PtrFormatter(usize);

		impl Debug for PtrFormatter {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				write!(f, "{:p}", self.0 as *const ())
			}
		}

		f.debug_struct("ObjectInfo")
		 .field("id", &self.id)
		 .field("env", &self.env)
		 .field("parent", &self.parent)
		 .field("hash", &PtrFormatter(self.hash as usize))
		 .finish()
	}
}

impl<T: Send + Sync + ?Sized> Clone for Object<T> {
	fn clone(&self) -> Object<T> {
		Object(self.0.clone())
	}
}

impl<T: Send + Sync + ?Sized> Eq for Object<T> {}
impl<T: Send + Sync + ?Sized> PartialEq for Object<T> {
	fn eq(&self, other: &Object<T>) -> bool {
		self.0 == other.0
	}
}

impl<T: Send + Sync + ?Sized> Eq for Inner<T> {}
impl<T: Send + Sync + ?Sized> PartialEq for Inner<T> {
	fn eq(&self, _other: &Inner<T>) -> bool {
		unimplemented!()
	}
}

impl Hash for AnyObject {
	fn hash<'a, H: Hasher>(&self, h: &'a mut H) {
		(self.0.info.hash)(&self.0.data, h);
	}
}

impl<T: 'static + Send + Sync + Sized> Hash for Object<T> {
	fn hash<'a, H: Hasher>(&self, h: &'a mut H) {
		// this is a really awkward way to do it, but whatever?
		// especially if T is hashable on its own, this might lead to weird situations
		(self.0.info.hash)(&self.0.data as &dyn Any, h);
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
		assert_eq!(obj.data(), &MyType(123));

		let obj = obj.as_any();
		let obj2: Object<MyType> = Object::new_child(MyType(456), obj.clone());

		assert_eq!(obj2.parent(), Some(obj));
		assert_eq!(obj2.data(), &MyType(456));
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






