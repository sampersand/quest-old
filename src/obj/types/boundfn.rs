use env::Environment;
use shared::Shared;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use obj::{Type, Object, AnyShared, AnyResult, SharedObject};
use obj::types::{HasDefaults, Var, IntoObject};

use std::fmt::{self, Debug, Display, Formatter};

pub trait Callable<T: ?Sized> : Send + Sync {
	fn call(self: &Self, obj: SharedObject<T>, args: &[AnyShared], env: &Environment) -> AnyResult;
}

impl<T: ?Sized, F: Fn(SharedObject<T>, &[AnyShared], &Environment) -> AnyResult + Send + Sync> Callable<T> for F {
	fn call(self: &Self, obj: SharedObject<T>, args: &[AnyShared], env: &Environment) -> AnyResult {
		self.call((obj, args, env))
	}
}

pub struct BoundFn<T: ?Sized> {
	obj: SharedObject<T>,
	func: Box<dyn Callable<T>>
}

impl<T: ?Sized> BoundFn<T> {
	pub fn new<F: Callable<T> + 'static>(obj: SharedObject<T>, func: F) -> Self {
		BoundFn { obj, func: Box::new(func) }
	}
}

impl<T: Send + Sync + 'static> Object<T> {
	pub fn bind_to(&self, func: fn(SharedObject<T>, &[AnyShared], &Environment) -> AnyResult) -> BoundFn<T> {
		BoundFn::new(self.upgrade(), func)
	}
}

impl<T: Send + Sync + 'static> HasDefaults for Object<BoundFn<T>> {
	fn get_default_var(&self, attr: &str, env: &Environment) -> Option<AnyResult> {
		match attr {
			"()" => Some(self.data.func.call(self.data.obj.clone(), &[], env)),
			// "()" => Some(Ok(
			// 		Object::new(self.bind_to(|b, args, env| {
			// 			let b = b.read();
			// 			b.data.func.call(b.data.obj.clone(), &[], env)
			// 		})) as AnyShared
			// 	)),
			_ => ::obj::types::any::get_default_norm(self, ::obj::Id::from_nonstatic_str(attr).into_anyshared(), env)
		}
	}
}

impl<T: Debug> Debug for BoundFn<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("BoundFn").field("obj", &self.obj).field("func", &(&self.func as *const _ as usize)).finish()
	}
}

impl<T: Display> Display for BoundFn<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "<bound fn for {}>", self.obj)
	}
}

impl<T: ?Sized + Eq> Eq for BoundFn<T> {}
impl<T: ?Sized + PartialEq> PartialEq for BoundFn<T> {
	fn eq(&self, other: &BoundFn<T>) -> bool {
		unimplemented!("== for bound fn")
	}
}
impl<T> ::std::hash::Hash for BoundFn<T> {
	fn hash<H: ::std::hash::Hasher>(&self, h: &mut H) {
		unimplemented!("hash for bound fn")
	}
}