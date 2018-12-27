mod object;
mod types;

pub use self::object::Object;

use std::any::Any;
use shared::Shared;

pub type AnyObject = Object<dyn Any + Send + Sync>;
pub type AnyShared = Shared<AnyObject>;
pub type SharedObject<T> = Shared<Object<T>>;

impl<T: Send + Sync + 'static> Object<T> {
	pub fn shared(self) -> SharedObject<T> {
		Shared::new(self)
	}

	#[inline]
	pub fn any(self) -> AnyShared {
		self.shared().any()
	}
}

impl<T: Send + Sync + 'static> SharedObject<T> {
	#[inline(always)]
	pub fn any(self) -> AnyShared {
		self as AnyShared
	}
}
