use obj::{Id, Result, AnyObject, SharedObject, classes::QuestClass};
use shared::{Shared, SharedMap};
use env::Environment;

use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Copy)]
struct Defaults {
	get: fn(&AnyObject, &Id) -> Option<AnyObject>,
	has: fn(&AnyObject, &Id) -> bool
}

pub struct QObject<C: Send + Sync + ?Sized> {
	id: Id,
	attrs: HashMap<Id, AnyObject>,
	defaults: Defaults,
	class: C
}

impl<C: Send + Sync> QObject<C> {
	pub fn make_shared(self) -> Shared<Self> {
		Shared::from(self)
	}
}

impl<C: Send + Sync> From<C> for SharedObject<C> where SharedObject<C>: QuestClass {
	#[inline]
	fn from(class: C) -> SharedObject<C> {
		SharedObject::new(QObject {
			id: Id::next(),
			attrs: HashMap::new(),
			defaults: Defaults{ get: Self::GET_DEFAULTS, has: Self::HAS_DEFAULTS },
			class,
		})
	}
}



impl<C: Send + Sync + Clone> SharedObject<C> {
	pub fn clone_object(&self) -> Self {
		QObject::clone(&*self.read()).into()
	}
}

impl<C: Send + Sync + ?Sized> Shared<QObject<C>> {
	pub fn id(&self) -> Id {
		self.read().id
	}
}

impl AnyObject {
	pub fn get_attr<Q: Borrow<Id>>(&self, attr: Q) -> Option<AnyObject> {
		let attr = attr.borrow();
		if let Some(obj) = self.read().attrs.get(attr) {
			Some(obj.clone())
		} else {
			(self.read().defaults.get)(self, attr)
		}
	}

	pub fn has_attr<Q: Borrow<Id>>(&self, attr: Q) -> bool {
		let attr = attr.borrow();
		self.read().attrs.contains_key(attr) || (self.read().defaults.has)(self, attr)
	}

	pub fn set_attr<Q: Into<Id>>(&self, attr: Q, obj: AnyObject) -> Option<AnyObject> {
		let ref mut attrs = self.write().attrs;
		let attr = attr.into();
		let prev = attrs.remove_entry(&attr).map(|(_, v)| v);
		assert!(attrs.insert(attr, obj).is_none());
		prev
	}

	pub fn del_attr<Q: Borrow<Id>>(&self, attr: Q) -> Option<AnyObject> {
		self.write().attrs.remove_entry(attr.borrow()).map(|(_, obj)| obj)
	}

	pub fn call<Q: Borrow<Id>>(&self, args: &[&AnyObject], env: &Environment) -> Result<AnyObject> {
		self.call_attr("()", args, env)
	}

	pub fn call_local<Q: Borrow<Id>>(&self, args: &[&AnyObject], env: &Environment) -> Result<AnyObject> {
		self.call_attr("{}", args, env)
	}

	pub fn call_attr<Q: Borrow<Id>>(&self, attr: Q, args: &[&AnyObject], env: &Environment) -> Result<AnyObject> {
		if let Some(attr) = self.get_attr(attr.borrow()) {
			Ok(unimplemented!())
		} else {
			panic!("Attribute `{}` doesn't exist for {:#?}", attr.borrow(), self)
		}
		// self.get_attr(attr)?.ca
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


impl<C: QuestClass> QObject<C> {
	fn get_default_attr(&self, attr: &Id) -> Option<AnyObject> {
		unimplemented!("TODO: Default attributes for QObject");
		// DEFAULT_ATTRS.get(attr)
	}

	fn has_default_attr(&self, attr: &Id) -> bool {
		unimplemented!("TODO: Default attributes for QObject");
		// DEFAULT_ATTRS.contains_key(attr)
	}
}


impl<C: Clone + Send + Sync + Sized> Clone for QObject<C> {
	fn clone(&self) -> Self {
		QObject {
			id: Id::next(), // if we clone ourself, we need a new Id
			attrs: self.attrs.clone(),
			class: self.class.clone(),
			defaults: self.defaults
		}
	}
}

impl<C: Send + Sync + Hash> Hash for QObject<C> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.id.hash(h)
	}
}

impl<C: Send + Sync + ?Sized> AsRef<C> for QObject<C> {
	#[inline]
	fn as_ref(&self) -> &C {
		&self.class
	}
}

impl<C: Send + Sync + ?Sized> Deref for QObject<C> {
	type Target = C;

	#[inline]
	fn deref(&self) -> &C {
		&self.class
	}
}

impl<C: Send + Sync + Display> Display for QObject<C> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", self.class)
	}
}

impl<C: Send + Sync + ?Sized> DerefMut for QObject<C> {
	#[inline]
	fn deref_mut(&mut self) -> &mut C {
		&mut self.class
	}
}

impl<C: Send + Sync + ?Sized> Eq for QObject<C> {}
impl<C: Send + Sync + ?Sized> PartialEq for QObject<C> {
	fn eq(&self, other: &QObject<C>) -> bool {
		self.id == other.id
	}
}

impl<C: Debug + Send + Sync + ?Sized> Debug for QObject<C> {
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


