mod typed;

pub use self::typed::TypedObject;

use crate::{Shared, Environment};
use crate::err::{Error, Result};
use crate::collections::{Collection, Mapping};

use std::any::TypeId;
use std::fmt::{self, Debug, Formatter};


pub type Object = Shared<ObjectInner>;

pub trait IntoObject {
	fn into_object(self) -> Object;
}

pub struct ObjectInner {
	id: usize,
	mapid: TypeId,
	map: Shared<dyn Mapping>,
	env: Shared<Environment>
}

impl ObjectInner {
	pub fn new<M: Mapping + 'static>(map: M) -> Self {
		use std::sync::atomic::{AtomicUsize, Ordering};
		lazy_static::lazy_static! {
			static ref ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
		}

		ObjectInner {
			id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
			mapid: TypeId::of::<M>(),
			map: Shared::new(map) as _,
			env: Environment::current()
		}
	}
}


impl Eq for Object {}
impl PartialEq for Object {
	fn eq(&self, other: &Object) -> bool {
		if self.read().map.ptr_eq(&other.read().map) {
			return true;
		}

		if let (Some(var1), Some(var2)) = (self.downcast_var(), other.downcast_var()) {
			var1 == var2
		} else {
			self.call(&TypedObject::new_var("==").objectify(), &[other])
			    .ok()
			    .and_then(Shared::<ObjectInner>::into_bool)
			    .unwrap_or(false)
		}
	}
}

impl Clone for ObjectInner {
	fn clone(&self) -> Self {
		unimplemented!()
	}
}

impl Debug for ObjectInner {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("ObjectInner")
		 .field("id", &self.id)
		 .field("map", &self.map)
		 .field("env", &self.env)
		 .finish()
	}
}

impl Object {
	pub fn call(&self, attr: &Object, args: &[&Object]) -> Result {
		let value = self.get(attr).ok_or_else(|| Error::MissingKey {
			key: attr.clone(),
			obj: self.clone()
		})?;
		let mut new_args = args.to_vec();
		new_args.insert(0, self);
		value.call_unbound(&new_args)
	}

	fn call_unbound(self, args: &[&Object]) -> Result {
		if let Some(rustfn) = self.downcast_rustfn() {
			rustfn.call(args)
		} else {
			self.call(&TypedObject::new_var("()").objectify(), args)
		}
	}
}

impl Collection for ObjectInner {
	fn len(&self) -> usize {
		self.map.len()
	}

	fn is_empty(&self) -> bool {
		self.map.is_empty()
	}
}

impl Mapping for ObjectInner {
	fn get(&self, key: &Object) -> Option<Object> {
		self.map.get(key)
	}

	#[inline]
	fn set(&mut self, key: Object, val: Object) -> Option<Object> {
		self.map.set(key, val)
	}

	#[inline]
	fn del(&mut self, key: &Object) -> Option<Object> {
		self.map.del(key)
	}

	#[inline]
	fn has(&self, key: &Object) -> bool {
		self.map.has(key)
	}
}
