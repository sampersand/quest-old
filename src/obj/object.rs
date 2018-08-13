use obj::{Id, Result, classes::{QuestClass, DefaultAttrs}};
use shared::{Shared, SharedMap};
use env::Environment;

use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Display, Formatter};

pub type SharedObject = Shared<QObject<dyn QuestClass>>;

pub struct QObject<C: ?Sized> {
	id: Id,
	attrs: HashMap<Id, SharedObject>,
	class: C
}

impl<C> QObject<C> {
	pub fn new(class: C) -> QObject<C> {
		QObject { id: Id::next(), class, attrs: HashMap::new() }
	}

	pub fn make_shared(self) -> Shared<Self> {
		Shared::from(self)
	}
}

impl<C: QuestClass> From<Shared<QObject<C>>> for SharedObject {
	#[inline]
	fn from(inp: Shared<QObject<C>>) -> SharedObject {
		inp as _
	}
}

impl<C: Clone + Sized> Clone for QObject<C> {
	fn clone(&self) -> Self {
		QObject {
			id: self.id,
			attrs: self.attrs.clone(),
			class: self.class.clone()
		}
	}
}

impl<C: Hash> Hash for QObject<C> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.id.hash(h)
	}
}

impl<C> From<C> for QObject<C> {
	#[inline]
	fn from(class: C) -> QObject<C> {
		QObject::new(class)
	}
}

impl<C: ?Sized> AsRef<C> for QObject<C> {
	#[inline]
	fn as_ref(&self) -> &C {
		&self.class
	}
}

impl<C: ?Sized> Deref for QObject<C> {
	type Target = C;

	#[inline]
	fn deref(&self) -> &C {
		&self.class
	}
}

default impl<C: Display> Display for QObject<C> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.class, f)
	}
}

impl<C: ?Sized> DerefMut for QObject<C> {
	#[inline]
	fn deref_mut(&mut self) -> &mut C {
		&mut self.class
	}
}

impl<C: ?Sized> Eq for QObject<C> {}
impl<C: ?Sized> PartialEq for QObject<C> {
	fn eq(&self, other: &QObject<C>) -> bool {
		self.id == other.id
	}
}

impl<C: ?Sized> Shared<QObject<C>> {
	pub fn id(&self) -> Id {
		self.read().id
	}

	pub fn set_attr<Q: Into<Id>>(&self, attr: Q, obj: SharedObject) -> Option<SharedObject> {
		let ref mut attrs = self.write().attrs;
		let attr = attr.into();
		let prev = attrs.remove_entry(&attr).map(|(_, v)| v);
		assert!(attrs.insert(attr, obj).is_none());
		prev
	}

	pub fn del_attr<Q: Borrow<Id>>(&self, attr: Q) -> Option<SharedObject> {
		self.write().attrs.remove_entry(attr.borrow()).map(|(_, obj)| obj)
	}

	pub fn call_attr<Q: Borrow<Id>>(&self, attr: Q, args: &[&SharedObject], env: &Environment) -> Result<SharedObject> {
		// self.get_attr(attr)?.ca
		unimplemented!()
		// if let Some(qboundfn) = self.attrs.get(id.clone(), self) {
		// 	if let Classes::BoundFn(boundfn) = qboundfn.class().deref() {
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

impl<C: QuestClass> Shared<QObject<C>> {
	pub fn get_attr<Q: Borrow<Id>>(&self, attr: Q) -> Option<SharedObject> {
		if let Some(obj) = self.read().attrs.get(attr.borrow()) {
			Some(obj.clone())
		} else {
			C::default_attrs().get(attr.borrow()).map(|x| unimplemented!())
		}
	}
	pub fn has_attr<Q: Borrow<Id>>(&self, attr: Q) -> bool {
		self.read().attrs.contains_key(attr.borrow()) || C::default_attrs().contains_key(attr.borrow())
	}
}

impl<C: QuestClass> QObject<C> {
	pub fn default_attrs() -> &'static DefaultAttrs<C> {
		// define_attrs!{ static ref DEFAULT_ATTRS for QObject<C>;

		// };
		// &DEFAULT_ATTRS
		unimplemented!("TODO: default attributes for QObject");
	}
}

impl<C: Debug + ?Sized> Debug for QObject<C> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let mut f = f.debug_struct("QObject");
		f.field("id", &self.id);

		if !self.attrs.is_empty() {
			f.field("attrs", &self.attrs.keys());
		}

		f.field("class", &&self.class);
		f.finish()
	}
}








