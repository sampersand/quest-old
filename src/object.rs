mod types;
mod map;
mod literal;

use self::types::Type;
use self::map::ObjectMap;
pub use self::literal::{Literal, consts as literals};

use std::sync::{Arc, RwLock, Weak};
use std::any::Any;
use crate::shared::Shared;
use crate::map::Map;
use std::convert::TryFrom;
use crate::err::{Error, Result};
use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Formatter};
use lazy_static::lazy_static;

pub struct Object<T: ?Sized + Send + Sync>(Arc<Inner<T>>);
pub type AnyObject = Object<dyn Any + Send + Sync>;

struct InternalOps {
	hash: fn(&dyn Any, &mut Hasher),
	debug: fn(&dyn Any, &mut Formatter) -> fmt::Result,
	duplicate: fn(&AnyObject, Option<AnyObject>) -> AnyObject
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
		Object::_new(data, None, crate::env::current())
	}

	#[cfg_attr(feature = "ignore-unused", allow(unused))]
	pub fn new_child(data: T, parent: AnyObject) -> Object<T> {
		Object::_new(data, Some(parent), crate::env::current())
	}

	fn _new(data: T, parent: Option<AnyObject>, env: Shared<dyn Map>) -> Object<T> {
		use std::sync::atomic::{AtomicUsize, Ordering};

		lazy_static! {
			static ref ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
		}

		let inner = Arc::new(Inner {
			map: Shared::new(ObjectMap::from_type::<T>()),
			info: ObjectInfo {
				id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
				env: env,
				parent: parent,
			},
			ops: InternalOps {
				hash: |obj, mut hasher| Any::downcast_ref::<T>(obj).expect("bad obj passed to `hash`").hash(&mut hasher),
				debug: |obj, f| Debug::fmt(Any::downcast_ref::<T>(obj).expect("bad obj passed to `debug`"), f),
				duplicate: |obj, parent| Object::_new(
					{
						let obj: &dyn Any = &*obj.data().read().expect("read err in InternalOps::duplicate");
						Any::downcast_ref::<T>(obj).expect("bad obj passed to `duplicate`").clone()
					},
					parent.or_else(|| obj.parent().clone()),
					obj.env().clone(),
				) as AnyObject
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

impl<T: Type + Clone + Sized> Object<T> {
	pub fn duplicate(&self) -> Object<T> {
		self.as_any().duplicate().downcast::<T>().expect("duplicate returned a different object type?")
	}
}

impl<T: Send + Sync + Clone> Object<T> {
	#[cfg(test)]
	pub fn unwrap_data(&self) -> T {
		self.data().read().expect("read err in Object::unwrap_clone_data").clone()
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
	pub fn env(&self) -> &Shared<dyn Map> {
		&self.0.info.env
	}

	#[cfg_attr(feature = "ignore-unused", allow(unused))]
	pub fn data(&self) -> &RwLock<T> {
		&self.0.data
	}

	pub fn map(&self) -> &Shared<ObjectMap> {
		&self.0.map
	}

	#[cfg(test)]
	unsafe fn data_ptr(&self) -> *const T {
		&*self.data().read().expect("read err in Object::data_ptr") as *const _
	}

	pub fn id_eq(&self, other: &Object<T>) -> bool {
		self.0.info.id == other.0.info.id
	}
}

impl AnyObject {
	pub fn call_attr(&self, attr: Literal, args: &[&AnyObject]) -> Result<AnyObject> {
		self.call(&Object::new_variable(attr).as_any(), args)
	}

	pub fn get_attr(&self, attr: Literal) -> Result<AnyObject> {
		self.get(&Object::new_variable(attr).as_any())
	}

	pub fn duplicate(&self) -> AnyObject {
		(self.0.ops.duplicate)(self, None)
	}

	pub fn duplicate_add_parent(&self, parent: AnyObject) -> AnyObject {
		(self.0.ops.duplicate)(self, Some(parent))
	}
}

impl AnyObject {
	pub fn call(&self, attr: &AnyObject, args: &[&AnyObject]) -> Result<AnyObject> {
		// if `self` is a RustFn (ie a function written in rust, not Quest)...
		if let Some(rustfn) = self.downcast::<types::RustFn>() {
			// ...and `attr` is a Variable...
			if let Some(var) = attr.downcast::<types::Variable>() {
				// ...and `attr` is `CALL` (ie we're calling `self`), then:
				if var == literals::CALL {
					// if we have a parent, have that be the first arg.
					if let Some(ref parent) = self.parent() {
						let mut parent_args: Vec<&AnyObject> = Vec::with_capacity(args.len() + 1);
						parent_args.push(parent);
						parent_args.extend(args);
						return rustfn.data().read().expect("err when calling rustfn").call(&parent_args);
					} else {
						// otherwise, just call it straight up
						return rustfn.data().read().expect("err when calling rustfn").call(args);
					}
				}
			}
		}

		self.get(attr)?.call_attr(literals::CALL, args)
	}

	pub fn get(&self, attr: &AnyObject) -> Result<AnyObject> {
		// TODO: check custom values, eg `id` `parent`, etc
		types::pristine::_colon_colon(self, &Object::new_variable(literals::ATTR_GET).as_any())?
			.call_attr(literals::CALL, &[self, attr])
	}


	pub fn set(&self, attr: AnyObject, val: AnyObject) {
		self.0.map.write().expect("write err in AnyObject::set").set(attr, val);
	}

	pub fn del(&self, attr: &AnyObject) -> Result<AnyObject> {
		self.0.map.write().expect("write err in AnyObject::del").del(attr)
			.ok_or_else(|| Error::AttrMissing { attr: attr.clone(), obj: self.clone() })
	}

	pub fn has(&self, attr: &AnyObject) -> bool {
		self.0.map.read().expect("get err in AnyObject::has").has(attr)
	}
}

impl<T: 'static + Send + Sync + Sized> Object<T> {
	pub fn as_any(&self) -> AnyObject {
		Object(self.0.clone() as _)
	}
}

impl AnyObject {
	pub fn is<T: Send + Sync + 'static>(&self) -> bool {
		self.0.data.read().unwrap().is::<T>()
	}

	pub fn downcast_ref<T: Send + Sync + 'static>(&self) -> Option<&Object<T>> {
		// im unsure if this is safe, but it seems so, as all tests passed. but this could be a future
		// source of error (ie not casting thru ptr)
		if self.is::<T>() {
			Some(unsafe { &*(self as *const AnyObject as *const Object<T>) })
		} else {
			None
		}
	}

	pub fn downcast_ref_or_err<T: Send + Sync + 'static>(&self) -> Result<&Object<T>> {
		self.downcast_ref::<T>().ok_or_else(|| Error::CastError { obj: self.clone(), into: type_name::get::<T>() })
	}

	pub fn downcast_or_err<T: Send + Sync + 'static>(&self) -> Result<Object<T>> {
		self.downcast::<T>().ok_or_else(|| Error::CastError {
			obj: self.clone(),
			into: type_name::get::<T>()
		})
	}

	pub fn downcast<T: Send + Sync + 'static>(&self) -> Option<Object<T>> {
		self.downcast_ref().cloned()
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
			let lhs = lhs.data().read().expect(const_concat!("lhs read err in AnyObject::", literals::EQL));
			let rhs = rhs.data().read().expect(const_concat!("rhs read err in AnyObject::", literals::EQL));
			*lhs == *rhs
		} else {
			self.call_attr(literals::EQL, &[rhs])
				.and_then(|obj| obj.to_boolean())
				.map(|x| x.is_true())
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


use std::{marker::Unsize, ops::CoerceUnsized};
impl<T: Unsize<U> + Send + Sync + ?Sized, U: Send + Sync + ?Sized> CoerceUnsized<Object<U>> for Object<T> {}
// impl<T: std::marker::Unsize<U> + Send + Sync + ?Sized, U: Send + Sync + ?Sized> std::ops::CoerceUnsized<Object<U>> for Object<T> {}


#[cfg(test)]
mod fn_tests {
	use super::*;
	use crate::object::types::Number;

	#[test]
	#[ignore]
	fn make_sure_attr_access_works() -> Result<()> {
		let ref one = Object::new_number(1.0).as_any();
		let ref two = Object::new_number(2.0).as_any();

		one.call_attr(literals::ATTR_SET, &[two])?;
		assert!(one.has(two));
		assert!(one.get(two)?.has(&Object::new_variable(literals::L_PARENT).as_any()));
		assert!(!two.has(&Object::new_variable(literals::L_PARENT).as_any()));

		Ok(())
		// two.call
		// let add = one.get_attr(literals::ADD)?;
		// let y = add.call_attr(literals::CALL, &[&Object::new_number(2.3).as_any()])?;
		// assert_eq!(y.downcast_or_err::<Number>()?, 3.5);
		// Ok(())
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::HashMap;

	#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
	struct MyType(i32);

	impl Type for MyType {
		fn get_type_map() -> Shared<dyn Map> {
			lazy_static! {
				static ref MAP: Shared<dyn Map> = {
					let mut map = crate::map::ParentMap::<HashMap<_, _>>::new_default(types::BASIC_MAP.clone());
					map.set(Object::new_variable(literals::EQL).as_any(), Object::new_rustfn::<_, MyType>(|obj, arg| {
						Ok(Object::new_boolean(arg[0].downcast::<MyType>().map(|x| obj.unwrap_data() == x.unwrap_data()).unwrap_or(false)))
					}));
					Shared::new(map)
				};
			}
			MAP.clone()
		}
	}

	// #[allow(unused)]
	fn _check_is_sized() {
		fn _is_sized<T: Sized>(){}
		_is_sized::<AnyObject>()
	}

	#[test]
	#[ignore]
	fn _make_sure_invalid_operations_are_ignored() {
		unimplemented!()
	}

	#[test]
	fn new() {
		let obj: Object<MyType> = Object::new(MyType(123));
		assert_eq!(obj.parent(), None);
		assert_eq!(obj.unwrap_data(), MyType(123));

		let obj = obj.as_any();
		let obj2: Object<MyType> = Object::new_child(MyType(456), obj.clone());

		assert_eq!(obj2.parent(), Some(obj));
		assert_eq!(obj2.unwrap_data(), MyType(456));
	}

	#[test]
	fn duplicate(){
		let parent = Object::new(MyType(0));
		let obj = Object::new_child(MyType(112), parent.clone());
		let dup = obj.duplicate();

		assert_eq!(obj.unwrap_data(), dup.unwrap_data());
		unsafe {
			assert_ne!(obj.data_ptr(), dup.data_ptr());
		}

		assert_eq!(obj.parent().unwrap().id(), parent.id());
		assert_eq!(dup.parent().unwrap().id(), parent.id());
		assert_ne!(obj.id(), dup.id());

		assert_ne!(&*obj.0.map.read().unwrap() as *const _, &*dup.0.map.read().unwrap() as *const _);
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






