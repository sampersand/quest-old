use super::{TypedObject, Type, Types};
use crate::Shared;
use crate::object::{Object, IntoObject};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Num(i64);

macro_rules! impl_from {
	($($ty:ty)*) => {
		$(
			impl From<$ty> for Num {
				fn from(num: $ty) -> Num {
					Num(num as i64)
				}
			}

			impl IntoObject for $ty {
				fn into_shared(self) -> Shared<Object> {
					TypedObject::new_num(self).objectify()
				}
			}
		)*
	}
}

impl_from!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128);

impl_typed_object!(Num, new_num, downcast_num);

impl_type! { for Num, downcast_fn=downcast_num;
	fn "@text" (this) {
		this.0.to_string().into_shared()
	}
}