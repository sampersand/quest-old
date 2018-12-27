use env::Environment;
use shared::Shared;
use std::hash::{Hash, Hasher};
use obj::{Type, Object, AnyShared, AnyResult, SharedObject};

use std::fmt::{self, Debug, Display, Formatter};

type InnerFn = (Fn(&[AnyShared], &Environment) -> AnyResult) + Send + Sync;

pub struct BoundFnOld(Box<InnerFn>);

impl<T: Send + Sync + ?Sized + 'static> SharedObject<T> {
	pub fn bind_to__<F: Send + Sync + 'static + Fn(Self, &[AnyShared], &Environment) -> AnyResult>(self, func: F) -> BoundFnOld {
		BoundFnOld(Box::new(move |args, env| func(self.clone(), args, env)))
	}
}

__impl_type! {
	for BoundFnOld, with self attr;

	fn "()" (this) env, args, {
		this.read().data.call_bound(args, env)
	}

	fn _ () {
		any::__get_default_typed(self, attr.clone()).or_else(|| any::__get_default_tostring(self, attr))
	}
}

impl BoundFnOld {
	pub fn bind_void(func: fn(&[AnyShared], &Environment) -> AnyResult) -> Self {
		BoundFnOld(Box::new(func))
	}

	fn as_ptr(&self) -> *const InnerFn {
		self.0.as_ref() as *const InnerFn
	}

	pub fn call_bound(&self, args: &[AnyShared], env: &Environment) -> AnyResult {
		(self.0)(args, env)
	}
}


impl Debug for BoundFnOld {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("BoundFnOld")
		 .field("fn", &"<bound function>")
		 .finish()
	}
}

impl Display for BoundFnOld {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "<todo: bound function>")
	}
}

impl Eq for BoundFnOld {}
impl PartialEq for BoundFnOld {
	fn eq(&self, other: &BoundFnOld) -> bool {
		self.as_ptr() == other.as_ptr()
	}
}

impl Hash for BoundFnOld {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_ptr().hash(h);
	}
}





