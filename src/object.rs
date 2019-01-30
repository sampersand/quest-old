mod r#type;
mod map;

use self::r#type::Type;
use self::map::ObjectMap;

use std::sync::{Arc, Weak};
use std::any::Any;
use crate::shared::Shared;
use crate::map::Map;

#[derive(Debug)]
pub struct Object<T: ?Sized + Send + Sync>(Arc<Inner<T>>);
pub type AnyObject = Object<dyn Any + Send + Sync>;

#[derive(Debug)]
struct ObjectInfo {
	id: usize,
	env: Shared<dyn Map>,
	parent: Option<AnyObject>
}

#[derive(Debug)]
struct Inner<T: ?Sized + Send + Sync> {
	map: Shared<ObjectMap>,
	info: ObjectInfo,
	weakref: Weak<Inner<dyn Any + Send + Sync>>,
	data: T
}

impl<T: Type + Sized> Object<T> {
	pub fn new(data: T) -> Object<T> {
		Object::_new(data, None)
	}

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
				parent: parent
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
	pub fn parent(&self) -> Option<AnyObject> {
		self.0.info.parent.clone()
	}
	// pub fn id(&self) -> usize {
	// 	self.0.info.id
	// }
	// pub fn env(&self) -> Shared<dyn Map> {
	// 	self.0.info.env.clone()
	// }
	pub fn data(&self) -> &T {
		&self.0.data
	}
}
impl<T: 'static + Send + Sync + Sized> Object<T> {
	fn as_any(&self) -> AnyObject {
		Object(self.0.clone() as _)
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

impl<T: std::marker::Unsize<U> + Send + Sync + ?Sized, U: Send + Sync + ?Sized> std::ops::CoerceUnsized<Object<U>> for Object<T> {}


#[cfg(test)]
mod tests {
	use super::*;
	#[derive(Debug, PartialEq, Eq)]
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
}




