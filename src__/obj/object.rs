use shared::Shared;
use obj::{Type, Id, AnyObject, AnyResult, SharedObject, AnyShared, WeakObject, attrs::Attributes};
use obj::types::HasDefaults;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::any::Any;
use std::cell::UnsafeCell;
use std::ops::{CoerceUnsized, Deref, DerefMut};
use std::borrow::Borrow;
use std::marker::Unsize;
use std::fmt::{self, Debug, Display, Formatter};
use std::{thread, mem, ptr};


pub struct Object<T: ?Sized>{
	obj: WeakObject,
	id: Id,
	pub attrs: Attributes,
	pub data: T,
}

unsafe impl<T: Sync + ?Sized> Send for Object<T> {}
unsafe impl<T: Send + Sync + ?Sized> Sync for Object<T> {}

impl<T: CoerceUnsized<U> + ?Sized, U> CoerceUnsized<Object<U>> for Object<T> where Object<T>: HasDefaults  {}

impl<T: Send + Sync + 'static> Object<T> where Object<T>: HasDefaults {
	pub fn new(data: T) -> SharedObject<T> {
		let attrs = Attributes {
			obj: unsafe{ mem::uninitialized() },
			map: Default::default(),
			defaults: |this, attr, env| this.downcast_ref::<T>().unwrap().get_default(attr.clone(), env)
		};

		let obj = Object::new_raw(data, attrs);
		unsafe {
			ptr::write(&mut obj.data().attrs.obj as *mut WeakObject, obj.downgrade() as WeakObject);
		}
		obj
	}
}

impl<T: Send + Sync + 'static> Object<T> {
	fn new_raw(data: T, attrs: Attributes) -> SharedObject<T> {
		let obj = SharedObject::new(Object {
			obj: unsafe{ mem::uninitialized() },
			id: Id::next(),
			attrs,
			data
		});

		unsafe {
			ptr::write(&mut obj.data().obj as *mut WeakObject, obj.downgrade() as WeakObject);
		}
		obj
	}
}

impl<T: Type + Default> Object<T> where Object<T>: HasDefaults {
	pub fn default() -> SharedObject<T> {
		Object::new(T::default())
	}
}

impl<T: ?Sized> Object<T> {
	pub fn id(&self) -> &Id {
		&self.id
	}
}

impl<T: 'static> Object<T> {
	pub fn upgrade(&self) -> SharedObject<T> {
		let shared = self.obj.upgrade().expect("Arc doesn't exist but its contents do?");
		assert!(shared.read().data.is::<T>(), "bad obj passed?");
		unsafe {
			Shared::from_raw(shared.into_raw().convert::<Object<T>>())
		}
	}
}

impl AnyObject {
	pub fn upgrade(&self) -> AnyShared {
		self.obj.upgrade().expect("Arc doesn't exist but its contents do?")
	}

	pub fn try_upgrade<T: 'static>(&self) -> Option<SharedObject<T>> {
		if !self.data.is::<T>() {
			None
		} else {
			self.downcast_ref::<T>().map(Object::<T>::upgrade)
		}
	}

	pub fn downcast_ref<T: 'static>(&self) -> Option<&Object<T>> {
		if self.data.is::<T>() {
			unsafe {
				Some(&*(self as *const AnyObject as *const Object<T>))
			}
		} else {
			None
		}
	}
}

impl Hash for AnyObject {
	fn hash<H: Hasher>(&self, h: &mut H) {
		unimplemented!();
		// (self.ops.hash)(self, Box::new(h));
	}
}

impl<T: Hash> Hash for Object<T> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		unimplemented!();
		// self.data.hash(h);
	}
}

impl<T: Clone + Send + Sync + 'static> Object<T> {
	pub fn duplicate(&self) -> SharedObject<T> {
		Object::new_raw(self.data.clone(), self.attrs.clone())
	}
}

impl Eq for AnyObject {}
impl PartialEq for AnyObject {
	fn eq(&self, other: &AnyObject) -> bool {
		unimplemented!();
		// (self.ops.eq)(self, other)
	}
}

impl<T: Eq> Eq for Object<T> where Object<T>: PartialEq {}
impl<T: PartialEq + Send + Sync + 'static> PartialEq<AnyObject> for Object<T> {
	fn eq(&self, other: &AnyObject) -> bool {
		if self as &AnyObject as *const AnyObject == other as *const AnyObject {
			return true;
		}
		other.downcast_ref::<T>().map(|o| self == o).unwrap_or(false)
	}
}

impl<T: PartialEq + Send + Sync + 'static> PartialEq<Object<T>> for Object<T> {
	fn eq(&self, other: &Object<T>) -> bool {
		if self as &AnyObject as *const AnyObject == other as *const AnyObject {
			return true;
		}
		self.data == other.data
	}
}


impl Debug for AnyObject {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		unimplemented!();
		// (self.ops.debug_fmt)(self, f)
	}
}

impl<T: Debug> Debug for Object<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let mut s = f.debug_struct("Object");
		s.field("id", &self.id);
		if !self.attrs.is_empty() {
			s.field("attrs", &self.attrs);
		}

		s.field("data", &self.data);
		s.finish()
	}
}

impl Display for AnyObject {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		unimplemented!();
		// (self.ops.display_fmt)(self, f)
	}
}

impl<T: Display> Display for Object<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", self.data)
	}
}

