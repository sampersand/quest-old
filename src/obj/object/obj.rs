use sync::SpinRwLock;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use obj::{Id, Exception, classes, attrs::*, object::Classes};
use env::Environment;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)] // might eq and Partial Eq be because of ID?
pub struct QObj {
	id: Id,
	class: SpinRwLock<Classes>,
	pub(super) attrs: Attributes,
}

impl Eq for QObj {}
impl PartialEq for QObj {
	fn eq(&self, other: &QObj) -> bool {
		*self.class.read() == *other.class.read()
	}
}

impl Hash for QObj {
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		self.class.read().hash(hasher);
	}
}

impl QObj {
	pub fn new<O: HasDefaultAttrs + Into<Classes>>(obj: O) -> QObj {
		QObj {
			id: Id::next(),
			class: SpinRwLock::new(obj.into()),
			attrs: Attributes::new(O::default_attrs())
		}
	}

	#[inline]
	pub fn id(&self) -> Id {
		self.id
	}

	#[inline]
	pub fn class<'a>(&'a self) -> impl Deref<Target=Classes> + 'a {
		self.class.read()
	}

	#[inline]
	pub fn class_mut<'a>(&'a self) -> impl DerefMut<Target=Classes> + 'a {
		self.class.write()
	}

	#[inline]
	pub fn into_class(self) -> Classes {
		self.class.into_inner()
	}
}

impl Clone for QObj {
	fn clone(&self) -> Self {
		QObj {
			id: Id::next(),
			attrs: self.attrs.clone(),
			class: self.class.try_clone().expect("unable to clone qobj")
		}
	}
}

impl Display for QObj {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&*self.class.read(), f)
	}
}

default_attrs! { for QObj;
	fn "@text" (this) {
		Ok(this.to_string().into())
	}

	fn "@bool" (this) {
		Ok(true.into())
	}

	fn "clone" (this) {
		Ok(this.clone().into())
	}

	fn "==" (this, rhs) {
		Ok((this == rhs.deref()).into())
	}

	fn "!=" (this, rhs) {
		Ok((this != rhs.deref()).into())
	}

	fn "." (_this, attr) with env vars obj {
		obj.get_attr(attr.clone()).ok_or_else(|| Exception::Missing(attr.clone()))
	}
}
