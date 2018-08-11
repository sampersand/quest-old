use env::Environment;
use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Display, Formatter};

use obj::QObject;


#[derive(Clone, Copy)]
pub struct RustFn(pub &'static str, pub fn(&QObject, &[&QObject], &Environment) -> QObject);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QBoundFn(QObject, &'static RustFn);

impl QBoundFn {
	#[inline]
	pub fn new(obj: QObject, rustfn: &'static RustFn) -> QBoundFn {
		QBoundFn(obj, rustfn)
	}

	pub fn call(&self, args: &[&QObject], env: &Environment) -> QObject {
		((self.1).1)(&self.0, args, env)
	}
}

impl Display for QBoundFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

default_attrs! { for QBoundFn, with variant BoundFn;
	use QObj;
	fn "()" (this) with env args {
		this.call(args, env)
	}
}


impl RustFn {
	pub fn into_bound(&'static self, obj: &QObject) -> QBoundFn {
		QBoundFn(obj.clone(), self)
	}
	pub fn call_from_null(&self, args: &[&QObject], env: &Environment) -> QObject {
		(self.1)(&().into(), args, env)
	}
}

impl Eq for RustFn {}
impl PartialEq for RustFn {
	fn eq(&self, other: &RustFn) -> bool { self.1 as usize == other.1 as usize }
}

impl Hash for RustFn {
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		(self.1 as usize).hash(hasher)
	}
}

impl Debug for RustFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "RustFn({} : {:p})", self.0, self.1 as usize as *const u8)
		} else {
			write!(f, "RustFn({})", self.0)
		}
	}
}
