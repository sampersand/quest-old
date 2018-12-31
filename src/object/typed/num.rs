use super::{TypedObject, Type, Types};
use crate::Shared;
use crate::object::{Object, IntoObject};
use crate::collections::{Mapping, ParentalMap};
use std::fmt::{self, Debug, Display, Formatter};
use lazy_static::lazy_static;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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

impl Debug for Num {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Num({:?})", self.0)
	}
}

impl AsRef<i64> for Num {
	fn as_ref(&self) -> &i64 {
		&self.0
	}
}

macro_rules! impl_conversion {
	($($ty:ty)*) => {
		$(
			impl From<$ty> for Num {
				fn from(num: $ty) -> Num {
					Num(num as i64)
				}
			}

			impl From<Num> for $ty {
				fn from(num: Num) -> $ty {
					num.0 as $ty
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

impl_conversion!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

impl_typed_object!(Num, new_num, downcast_num, is_num);
impl_quest_conversion!("@num" (as_num_obj is_num) (into_num downcast_num) -> Num);

macro_rules! binary_oper {
	($this:ident $oper:tt $rhs:ident) => (($this.0 $oper $rhs.into_num()?.0).into_object())
}

impl_type! { for Num, downcast_fn=downcast_num;
	fn "@num" (this) {
		this.into_object()
	}

	fn "@bool" (this) {
		(this.0 != 0).into_object()
	}

	fn "@text" (this) {
		this.0.to_string().into_object()
	}

	fn "+" (this, rhs) { binary_oper!(this + rhs) }
	fn "-" (this, rhs) { binary_oper!(this - rhs) }
	fn "*" (this, rhs) { binary_oper!(this * rhs) }
	fn "/" (this, rhs) { binary_oper!(this / rhs) }
	fn "%" (this, rhs) { binary_oper!(this % rhs) }
	fn "^" (@this, rhs) { this.call_attr("**", &[rhs])? }
	fn "**" (this, rhs) {
		let rhs = rhs.into_num()?.0;
		debug!("TODO: change `**` to be able to accept fractions");
		this.0.pow(rhs as u32).into_object()
	}

	fn "==" (this, rhs) { binary_oper!(this == rhs) }
	fn "<" (this, rhs) { binary_oper!(this < rhs) }
	fn "<=" (this, rhs) { binary_oper!(this <= rhs) }
	fn ">" (this, rhs) { binary_oper!(this > rhs) }
	fn ">=" (this, rhs) { binary_oper!(this >= rhs) }

	fn "<=>" (this, rhs) {
		let (this, rhs) = (this.0, rhs.into_num()?.0);
		if this < rhs { (-1).into_object() } // yes i can match with `cmp`, but that's a pain
		else if this == rhs { 0.into_object() }
		else { 1.into_object() }
	}

	fn "bitand" (this, rhs) { binary_oper!(this & rhs) }
	fn "bitor" (this, rhs) { binary_oper!(this | rhs) }
	fn "bitxor" (this, rhs) { binary_oper!(this ^ rhs) }
	fn "bitshl" (this, rhs) { binary_oper!(this << rhs) }
	fn "bitshr" (this, rhs) { binary_oper!(this >> rhs) }
}





















