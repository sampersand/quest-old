use env::Environment;
use shared::Shared;
use std::hash::{Hash, Hasher};
use obj::{Type, Object, AnyShared, AnyResult, SharedObject};

use std::fmt::{self, Debug, Display, Formatter};

type InnerFn = (Fn(&[AnyShared], &Environment) -> AnyResult) + Send + Sync;

pub struct BoundFn(Box<InnerFn>);

impl<T: Send + Sync + ?Sized + 'static> SharedObject<T> {
	pub fn bind_to(self, func: fn(Self, &[AnyShared], &Environment) -> AnyResult) -> BoundFn {
		BoundFn(Box::new(move |args, env| func(self.clone(), args, env)))
	}
}

impl_type! {
	for BoundFn, with self attr;

	fn "()" (this) env, args, {
		this.read().data.call_bound(args, env)
	}

	fn _ () {
		any::get_default_attr_typed(self, attr).or_else(|| any::get_default_attr_tostring(self, attr))
	}
}

impl BoundFn {
	pub fn bind_void(func: fn(&[AnyShared], &Environment) -> AnyResult) -> Self {
		BoundFn(Box::new(func))
	}

	fn as_ptr(&self) -> *const InnerFn {
		self.0.as_ref() as *const InnerFn
	}

	pub fn call_bound(&self, args: &[AnyShared], env: &Environment) -> AnyResult {
		(self.0)(args, env)
	}
}


impl Debug for BoundFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("BoundFn")
		 .field("fn", &"<bound function>")
		 .finish()
	}
}

impl Display for BoundFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "<todo: bound function>")
	}
}

impl Eq for BoundFn {}
impl PartialEq for BoundFn {
	fn eq(&self, other: &BoundFn) -> bool {
		self.as_ptr() == other.as_ptr()
	}
}

impl Hash for BoundFn {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_ptr().hash(h);
	}
}






