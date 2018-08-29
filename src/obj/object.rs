use shared::Shared;
use obj::{Type, Id, AnyObject, AnyResult, SharedObject, AnyShared, WeakObject, attrs::Attributes};
use env::Environment;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::any::Any;
use std::cell::UnsafeCell;
use std::ops::{CoerceUnsized, Deref, DerefMut};
use std::borrow::Borrow;
use std::marker::Unsize;
use std::fmt::{self, Debug, Display, Formatter};
use std::{thread, mem, ptr};


struct Ops {
	debug_fmt: fn(&AnyObject, &mut Formatter) -> fmt::Result,
	display_fmt: fn(&AnyObject, &mut Formatter) -> fmt::Result,
	eq: fn(&AnyObject, &AnyObject) -> bool,
	hash: fn(&AnyObject, Box<&mut dyn Hasher>)

}

pub struct Object<T: ?Sized>{
	obj: WeakObject,
	id: Id,
	pub attrs: Attributes,
	ops: Ops,
	pub(super) data: T,
}

unsafe impl<T: Sync + ?Sized> Send for Object<T> {}
unsafe impl<T: Send + Sync + ?Sized> Sync for Object<T> {}

impl<T: CoerceUnsized<U>, U> CoerceUnsized<Object<U>> for Object<T> {}

impl<T: Debug + PartialEq + Hash + 'static> Object<T> where Object<T>: Type {
	pub fn new(data: T) -> SharedObject<T> {
		let object = SharedObject::new(Object {
			obj: unsafe{ mem::uninitialized() },
			id: Id::next(),
			attrs: Attributes {
				obj: unsafe{ mem::uninitialized() },
				map: Default::default(),
				defaults: |this, attr| this.downcast_ref::<T>().unwrap().get_default_attr(attr)
			},
			ops: Ops {
				debug_fmt: |this, f| this.downcast_ref::<T>().unwrap().debug_fmt(f),
				display_fmt: |this, f| this.downcast_ref::<T>().unwrap().display_fmt(f),
				eq: |this, o| this.downcast_ref::<T>().unwrap() == o,
				hash: |this, mut h| this.downcast_ref::<T>().unwrap().data.hash(&mut *h) 
			},
			data: data
		});

		unsafe {
			ptr::write(&mut object.data().obj as *mut WeakObject, object.downgrade() as WeakObject);
			ptr::write(&mut object.data().attrs.obj as *mut WeakObject, object.downgrade() as WeakObject);
		}

		object
	}
}

impl<T: ?Sized> Object<T> {
	pub fn data(&self) -> &T {
		&self.data
	}

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

impl<T: ?Sized> SharedObject<T> {
	pub fn read_call(&self, attr: &'static str, args: &[&AnyShared], env: &mut Environment) -> AnyResult {
		let func = self.read().attrs.get(attr)?;
		let r = func.read();
		r.attrs.call("()", args, env)
	}
}

impl<T> Deref for Object<T> {
	type Target = T;

	#[inline]
	fn deref(&self) -> &T {
		&self.data
	}
}

impl<T> DerefMut for Object<T> {
	#[inline]
	fn deref_mut(&mut self) -> &mut T {
		&mut self.data
	}
}

impl Hash for AnyObject {
	fn hash<H: Hasher>(&self, h: &mut H) {
		(self.ops.hash)(self, Box::new(h));
	}

}
impl<T: Hash> Hash for Object<T> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.data.hash(h);
	}
}

impl<T: Debug + PartialEq + Hash + Clone + 'static> Object<T> where Object<T>: Type {
	pub fn duplicate(&self) -> SharedObject<T> {
		Object::new(self.data.clone())
	}
}

impl Eq for AnyObject {}
impl PartialEq for AnyObject {
	fn eq(&self, other: &AnyObject) -> bool {
		(self.ops.eq)(self, other)
	}
}

impl<T: Eq> Eq for Object<T> where Object<T>: PartialEq {}
impl<T: 'static + PartialEq> PartialEq<AnyObject> for Object<T> {
	fn eq(&self, other: &AnyObject) -> bool {
		other.downcast_ref::<T>().map(|o| self.data == o.data).unwrap_or(false)
	}
}

impl Debug for AnyObject {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		(self.ops.debug_fmt)(self, f)
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
		(self.ops.display_fmt)(self, f)
	}
}

impl<T: Display> Display for Object<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", self.data)
	}
}

