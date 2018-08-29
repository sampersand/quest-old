use env_::Environment__;
use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Display, Formatter};

use obj_::{QObject__, Result_};


#[derive(Clone, Copy)]
pub struct RustFn(pub &'static str, pub fn(&QObject__, &[&QObject__], &Environment__) -> Result_);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QBoundFn(QObject__, &'static RustFn);

impl QBoundFn {
	#[inline]
	pub fn new(obj: QObject__, rustfn: &'static RustFn) -> QBoundFn {
		QBoundFn(obj, rustfn)
	}

	pub fn call(&self, args: &[&QObject__], env: &Environment__) -> Result_ {
		// todo: work out the difference betwixt these
		((self.1).1)(&self.0, args, env)
	}

	pub fn call_local(&self, args: &[&QObject__], env: &Environment__) -> Result_ {
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
	fn "{}" (this) with env args {
		this.call_local(args, env)
	}
}


impl RustFn {
	pub fn into_bound(&'static self, obj: &QObject__) -> QBoundFn {
		QBoundFn(obj.clone(), self)
	}
	pub fn call_from_null(&self, args: &[&QObject__], env: &Environment__) -> Result_ {
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
