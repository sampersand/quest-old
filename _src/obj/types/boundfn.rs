use shared::Shared;
use env::{Binding, Parent};
use obj::{Object, SharedObject, AnyShared};


type Inner<T> = fn(&SharedObject<T>, &[AnyShared]) -> AnyShared;

pub struct BoundFn<T: ?Sized>(Inner<T>);

impl<T: ?Sized> BoundFn<T> {
	#[inline(always)]
	pub fn new(func: Inner<T>) -> Self {
		BoundFn(func)
	}
}

impl<T: ?Sized> Object<BoundFn<T>> {
	pub fn new_bound(func: Inner<T>) -> Self {
		Object::new(BoundFn::from(func))
	}
}

impl<T: ?Sized> From<Inner<T>> for BoundFn<T> {
	fn from(func: Inner<T>) -> Self {
		BoundFn::new(func)
	}
}

impl<T: ?Sized> Parent for BoundFn<T> {
	fn binding() -> Shared<Binding> {
		Binding::empty()
		// DEFAULT_BOOL_ATTRS.clone()
	}
}
