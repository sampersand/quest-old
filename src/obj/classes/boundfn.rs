use std::any::Any;

use env::Environment;
use obj::{SharedObject, AnyObject, Result};
use shared::Shared;
use std::fmt::{self, Debug, Display, Formatter};


#[derive(Clone, Copy)]
pub struct BindableFn(pub fn(&AnyObject, &[&AnyObject], &Environment) -> Result<AnyObject>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundFn(BindableFn, AnyObject);

pub type QBoundFn = SharedObject<BoundFn>;

impl BindableFn {
	pub fn bind(self, obj: AnyObject) -> QBoundFn {
		QBoundFn::from(BoundFn(self, obj))
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


define_attrs! {
	static ref DEFAULT_ATTRS for QBoundFn;
	use QObject<BindableFn>;

	fn "{}" (this) with env args {
		unimplemented!("TODO: local call qblock");
	}

	fn "()" (this) with env args {
		unimplemented!("TODO: call qblock");
	}
}