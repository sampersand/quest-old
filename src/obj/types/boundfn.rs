use env::Environment;
use shared::Shared;
use std::hash::{Hash, Hasher};
use obj::{Type, Object, AnyShared, AnyResult, SharedObject};

use std::fmt::{self, Debug, Formatter};

type InnerFn = Fn(&[&AnyShared], &mut Environment) -> AnyResult;
pub struct BoundFn(Box<InnerFn>);

impl<T: ?Sized + 'static> SharedObject<T> {
	pub fn bind_to_shared(self, func: fn(&Self, &[&AnyShared], &mut Environment) -> AnyResult) -> BoundFn {
		BoundFn(Box::new(move |args, env| func(&self, args, env)))
	}

	pub fn bind_to(self, func: fn(&Object<T>, &[&AnyShared], &mut Environment) -> AnyResult) -> BoundFn {
		BoundFn(Box::new(move |args, env| func(&*self.read(), args, env)))
	}

	pub fn bind_to_mut(self, func: fn(&mut Object<T>, &[&AnyShared], &mut Environment) -> AnyResult) -> BoundFn {
		BoundFn(Box::new(move |args, env| func(&mut *self.write(), args, env)))
	}
}

impl Type for Object<BoundFn> {
	fn get_default_attr(&self, attr: &str) -> Option<BoundFn> {
		match attr {
			"()" => Some(self.upgrade().bind_to(|obj, args, env| obj.call_bound(args, env))),
			_ => unimplemented!()
		}
	}

	fn debug_fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{:?}", self.data())
	}

	fn display_fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "<bound fn>")
	}

}

impl BoundFn {
	pub fn bind_void(func: fn(&[&AnyShared], &mut Environment) -> AnyResult) -> Self {
		BoundFn(Box::new(func))
	}

	fn as_ptr(&self) -> *const InnerFn {
		self.0.as_ref() as *const InnerFn
	}

	pub fn call_bound(&self, args: &[&AnyShared], env: &mut Environment) -> AnyResult {
		(self.0)(args, env)
	}
}


impl Debug for BoundFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("BoundFn")
		 .field("fn", &"<todo>")
		 .finish()
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






