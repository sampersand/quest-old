use crate::{Shared, Environment};
use crate::err::{Error, Result};
use crate::collections::{Collection, Mapping};
use super::IntoObject;

use std::any::TypeId;
use std::fmt::{self, Debug, Display, Formatter};
use std::sync::Arc;
use lazy_static::lazy_static;

// note how there isn't a RwLock on the InnerObject
// that's because it doesn't need one--map and env can change on their own,
// and id / mapid aren't going to ever change
#[derive(Clone)]
#[cfg_attr(feature = "fine-debug", derive(Debug))]
pub struct Object(Arc<InnerObject>);

#[cfg_attr(feature = "fine-debug", derive(Debug))]
struct InnerObject {
	id: usize,
	mapid: TypeId,
	map: Shared<dyn Mapping>,
	env: Shared<Environment>
}

impl Object {
	pub fn new<M: Mapping + 'static>(map: M) -> Self {
		Object::new_with_env(map, Environment::current())
	}

	pub fn new_with_env<M: Mapping + 'static>(map: M, env: Shared<Environment>) -> Self {
		use std::sync::atomic::{AtomicUsize, Ordering};
		lazy_static! {
			static ref ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
		}

		Object(Arc::new(InnerObject {
			id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
			mapid: TypeId::of::<M>(),
			map: Shared::new(map) as _,
			env
		}))
	}

	pub fn ptr_eq(&self, rhs: &Object) -> bool {
		Arc::ptr_eq(&self.0, &rhs.0)
	}

	pub fn id(&self) -> usize  {
		self.0.id
	}

	pub fn map(&self) -> &Shared<dyn Mapping> {
		&self.0.map
	}

	pub fn env(&self) -> &Shared<Environment> {
		&self.0.env
	}

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
			self.call_attr("()", args)
		}
	}
	
	pub fn duplicate(&self) -> Object {
		Object::new_with_env(self.0.map.duplicate(), self.0.env.clone())
	}
}

impl Object {
	pub fn get_attr(&self, attr: &'static str) -> Option<Object> {
		self.get(&attr.into_object())
	}
	pub fn set_attr(&mut self, attr: &'static str, val: Object) -> Option<Object> {
		self.set(attr.into_object(), val)
	}
	pub fn del_attr(&mut self, attr: &'static str) -> Option<Object> {
		self.del(&attr.into_object())
	}
	pub fn has_attr(&self, attr: &'static str) -> bool {
		self.has(&attr.into_object())
	}
	pub fn call_attr(&self, attr: &'static str, args: &[&Object]) -> Result {
		self.call(&attr.into_object(), args)
	}
}

impl IntoObject for Object {
	fn into_object(self) -> Object {
		self
	}
}

impl Eq for Object {}
impl PartialEq for Object {
	fn eq(&self, other: &Object) -> bool {
		if Arc::ptr_eq(&self.0, &other.0) || self.0.map.ptr_eq(&other.0.map) {
			return true;
		}

		if let (Some(var1), Some(var2)) = (self.downcast_var(), other.downcast_var()) {
			var1 == var2
		} else {
			self.call_attr("==", &[other])
			    .and_then(|obj| obj.into_bool())
			    .map(|x| x.into_inner())
			    .unwrap_or(false)
		}
	}
}

impl Display for Object {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "Object({})", &*self.0.map.read())
		} else {
			write!(f, "{}", &*self.0.map.read())
		}
	}
}

#[cfg(not(feature = "fine-debug"))]
impl Debug for Object {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Object")
			 .field("id", &self.0.id)
			 .field("map", &self.0.map)
			 .finish()
		} else {
			write!(f, "Object({:?})", &*self.0.map.read())
		}
	}
}


impl Collection for Object {
	fn len(&self) -> usize {
		self.0.map.read().len()
	}

	fn is_empty(&self) -> bool {
		self.0.map.read().is_empty()
	}
}

impl Mapping for Object {
	fn duplicate(&self) -> Shared<dyn Mapping> {
		Shared::new(<Object>::duplicate(self)) as _
	}

	fn get(&self, key: &Object) -> Option<Object> {
		self.0.map.read().get(key)
	}

	#[inline]
	fn set(&mut self, key: Object, val: Object) -> Option<Object> {
		self.0.map.write().set(key, val)
	}

	#[inline]
	fn del(&mut self, key: &Object) -> Option<Object> {
		self.0.map.write().del(key)
	}

	#[inline]
	fn has(&self, key: &Object) -> bool {
		self.0.map.read().has(key)
	}
}
