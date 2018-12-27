use std::borrow::Borrow;
use env::{Binding, Parent};
use err::Result;
use obj::AnyShared;
use shared::Shared;
use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Display, Formatter};

/* MRO (method resolution order):

1. User-defined values (eg `foo.my_value`)
2. Specials (ie `@env`, `@attrs` or `@data` (possibly not `@data`?))
3. Class-defined values (eg `list.get(2)`)
4. Class parents
*/

#[derive(Clone)]
pub struct Object<T: ?Sized> {
	pub env: Shared<Binding>,
	pub attrs: Shared<Binding>,
	pub data: T
}

impl<T: Parent> Object<T> {
	#[inline]
	pub fn new(data: T) -> Self {
		Object {
			env: Binding::current(),
			attrs: Binding::new_child::<T>(),
			data
		}
	}

	pub fn new_shared(data: T) -> Shared<Self> {
		Shared::new(Object::new(data))
	}
}

impl<T: ?Sized> Object<T> {
	pub fn get_attr(&self, attr: &AnyShared) -> Option<AnyShared> {
		self.attrs.read().get(attr)
	}

	pub fn set_attr(&self, attr: AnyShared, val: AnyShared) {
		self.attrs.write().set(attr, val);
	}

	pub fn del_attr(&self, attr: &AnyShared) -> Option<AnyShared> {
		self.attrs.write().del(attr)
	}

	pub fn has_attr(&self, attr: &AnyShared) -> bool {
		self.attrs.read().has(attr)
	}

	pub fn call_attr(&self, attr: &AnyShared, args: &[AnyShared]) -> Option<Result<AnyShared>> {
		let obj = self.get_attr(attr)?;
		unimplemented!("TODO: CALL")
	}
}

impl<T: Parent> From<T> for Object<T> {
	fn from(data: T) -> Self {
		Object::new(data)
	}
}

impl<T: ?Sized> Eq for Object<T> {}
impl<T: ?Sized, O: ?Sized> PartialEq<Object<O>> for Object<T> {
	fn eq(&self, other: &Object<O>) -> bool {
		self.call_attr(&Object::new_var("==").any(), &[unimplemented!("{:?}", other)])
			.and_then(Result::ok)
			.and_then(|x| x.cast_bool().ok().map(|x| x.read().data))
			.unwrap_or(false)
	}
}

impl<T: ?Sized> Hash for Object<T> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.call_attr(&Object::new_var("hash").any(), &[])
			.and_then(Result::ok)
			.map(|x| {unimplemented!(); 1i32})
			.unwrap_or(self as *const _ as *const () as usize as i32)
			.hash(h);
	}
}

impl<T: ?Sized> Debug for Object<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let data_str = self.call_attr(&Object::new_var("inspect").any(), &[])
			.and_then(Result::ok)
			.and_then(|x| x.cast_text().ok().map(|x| x.read().data.clone()))
			.unwrap_or_else(|| )
			.expect("Couldn't call `inspect` and get a repr");
		f.debug_struct("Object")
		 .field("env", &self.env)
		 .field("attrs", &self.attrs)
		 .field("data", &data_str)
		 .finish()
	}
}





