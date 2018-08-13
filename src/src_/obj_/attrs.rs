use sync_::SyncMap;
use std::collections::HashMap;
use std::borrow::Borrow;

use obj_::{Id, QObject__, Classes};
use obj_::classes_::boundfn::RustFn;
use std::fmt::{self, Debug, Display, Formatter};

pub type DefaultAttrs = HashMap<Id, RustFn>;

pub trait HasDefaultAttrs {
	fn default_attrs() -> &'static DefaultAttrs;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AttrId {
	Id(Id),
	Obj(QObject__)
}

pub(super) struct Attributes {
	assigned: SyncMap<AttrId, QObject__>,
	defaults: &'static DefaultAttrs
}



impl<I: Into<Id>> From<I> for AttrId {
	#[inline]
	fn from(id: I) -> AttrId {
		AttrId::Id(id.into())
	}
}

impl From<QObject__> for AttrId {
	#[inline]
	fn from(obj: QObject__) -> AttrId {
		if let Classes::Var(var) = *obj.class() {
			return AttrId::Id((*var).into());
		}

		AttrId::Obj(obj)
	}
}

impl Display for AttrId {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			AttrId::Id(id) => write!(f, "<Id {}>", id),
			AttrId::Obj(obj) => write!(f, "<Obj {}>", obj)
		}
	}
}
impl Debug for Attributes {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Attributes").field("assigned", &self.assigned).field("defaults", &self.defaults).finish()
		} else {
			f.debug_tuple("Attributes").field(&self.assigned.read().keys()).field(&self.defaults.keys()).finish()
		}
	}
}

impl Clone for Attributes {
	fn clone(&self) -> Attributes {
		Attributes {
			assigned: self.assigned.try_clone().expect("deadlock for attribute clone"),
			defaults: self.defaults
		}
	}
}


impl Attributes {
	pub fn new(defaults: &'static DefaultAttrs) -> Attributes {
		Attributes { assigned: SyncMap::new(), defaults }
	}

	pub(super) fn has<I: Into<AttrId>>(&self, id: I) -> bool {
		let id = id.into();
		if self.assigned.has_key(&id) {
			return true;
		}

		if let AttrId::Id(id) = id {
 			self.defaults.contains_key(&id)
		} else {
			false
		}
	}

	pub(super) fn set<I: Into<AttrId>>(&self, id: I, val: QObject__) -> Option<QObject__> {
		self.assigned.set(id.into(), val)
	}

	pub(super) fn get<I: Into<AttrId>>(&self, id: I, caller: &QObject__) -> Option<QObject__> {
		let id = id.into();
		if let Some(val) = self.assigned.get(&id) { 
			return Some(val.clone());
		}

		let mut lock = self.assigned.write();

		if let Some(val) = lock.get(&id){ 
			return Some(val.clone());
		}

		if let AttrId::Id(attr_id) = id {
			let rustfn = self.defaults.get(&attr_id)?;
			Some(rustfn.into_bound(caller).into())
		} else {
			drop(lock);
			warn!("Attribute `{}` doesn't exist in {:?}", id, self);
			None
		}
	}

	pub fn del<I: Into<AttrId>>(&self, id: I) -> Option<QObject__> {
		self.assigned.del(&id.into())
	}
}