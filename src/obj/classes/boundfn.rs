use std::any::Any;

use env::Environment;
use obj::{SharedObject, AnyObject, Result};
use std::fmt::{self, Debug, Display, Formatter};


#[derive(Clone, Copy)]
pub struct BindableFn(pub fn(AnyObject, &[&AnyObject], &Environment) -> Result<AnyObject>);

#[derive(Debug, Clone)]//, PartialEq, Eq)]
pub struct BoundFn(BindableFn, AnyObject);

pub type QBoundFn = SharedObject<BoundFn>;

impl QBoundFn {
	pub fn call(&self, args: &[&AnyObject], env: &Environment) -> Result<AnyObject> {
		let r = self.read();
		((r.data().0).0)(r.data().1.clone(), args, env)
	}
}

impl BindableFn {
	pub fn bind_to(self, obj: AnyObject) -> QBoundFn {
		QBoundFn::from(BoundFn(self, obj))
	}
}

impl Eq for BoundFn {}
impl PartialEq for BoundFn {
	fn eq(&self, other: &BoundFn) -> bool {
		self.0 == other.0 && unimplemented!("TODO: get eq for AnyObject working")//&self.1 == &other.1
	}
}


impl Eq for BindableFn {}
impl PartialEq for BindableFn {
	fn eq(&self, other: &BindableFn) -> bool {
		self.0 as usize == other.0 as usize
	}
}

impl Debug for BindableFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("BindableFn")
		 .field(&(self.0 as usize))
		 .finish()
	}
}

impl Display for BindableFn {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{:p}", self.0 as *const u8)
	}
}


define_attrs! { for QBoundFn;
	use QObject<BindableFn>;

	fn "{}" (this) with env args obj {
		unimplemented!("TODO: local call qblock");
		Ok(obj.clone())
	}

	fn "()" (this) with env args obj {
		unimplemented!("TODO: call qblock");
		Ok(obj.clone())
	}
}