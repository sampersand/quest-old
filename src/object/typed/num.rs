use super::{TypedObject, Type, Types};
use crate::Shared;
use crate::object::{Object, IntoObject};
use crate::collections::{Mapping, ParentalMap};
use std::fmt::{self, Display, Formatter};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Num(i64);

impl Num {
	pub fn new(num: i64) -> Num {
		Num(num)
	}
	pub fn into_inner(self) -> i64 {
		self.0
	}
}

impl Display for Num {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}


macro_rules! impl_from {
	($($ty:ty)*) => {
		$(
			impl From<$ty> for Num {
				fn from(num: $ty) -> Num {
					Num(num as i64)
				}
			}

			impl IntoObject for $ty {
				fn into_object(self) -> Object {
					TypedObject::new_num(self).objectify()
				}
			}
		)*
	}
}

impl_from!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128);

impl_typed_object!(Num, new_num, downcast_num, is_num);
impl_quest_conversion!(as_num -> Num, "@num" downcast_num);

impl_type! { for Num, downcast_fn=downcast_num;
	fn "@num" (@this) {
		assert!(this.is_num(), "called @num without a num");
		this.clone()
	}

	fn "@text" (this) {
		this.0.to_string().into_object()
	}

	fn "+" (this, rhs) {
		let rhs = rhs.as_num()?;
		(this.0 + rhs.0).into_object()
	}
}





















