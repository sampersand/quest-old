use super::{TypedObject, Type, Types};
use crate::Shared;
use crate::object::{Object, IntoObject};
use crate::collections::{Mapping, ParentalMap};
use std::fmt::{self, Debug, Display, Formatter};
use lazy_static::lazy_static;

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Number(f64);

impl Number {
	pub fn new(num: f64) -> Number {
		Number(num)
	}

	pub fn is_integer(&self) -> bool {
		(self.0 as u64 as f64) == self.0
	}

	pub fn into_integer(self) -> Number {
		Number((self.0 as u64) as f64)
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl Debug for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Number({:?})", self.0)
	}
}

impl AsRef<f64> for Number {
	fn as_ref(&self) -> &f64 {
		&self.0
	}
}

macro_rules! impl_conversion {
	($($ty:ty)*) => {
		$(
			impl From<$ty> for Number {
				fn from(num: $ty) -> Number {
					Number(num as f64)
				}
			}

			impl From<Number> for $ty {
				fn from(num: Number) -> $ty {
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

impl_conversion!(
	i8 i16 i32 i64 i128 isize
	u8 u16 u32 u64 u128 usize
	f32 f64
);

impl_typed_object!(Number, new_num, downcast_num, is_num);
impl_quest_conversion!("@num" (as_num_obj is_num) (into_num downcast_num) -> Number);

macro_rules! binary_oper {
	($this:ident $oper:tt $rhs:ident) => (($this.0 $oper $rhs.into_num()?.0).into_object());
	(bitwise; $this:ident $oper:tt $rhs:ident) => {
		if !$this.is_integer() {
			return Err(BadArgument {
				func: function!(),
				msg: "lhs isn't a whole number",
				position: 0,
				obj: unimplemented!("TODO: $this.clone()")
			})
		} else if !$rhs.into_num()?.is_integer()  {
			return Err(BadArgument {
				func: function!(),
				msg: "rhs isn't a whole number",
				position: 0,
				obj: unimplemented!("TODO: $rhs.clone()")
			})
		} else {
			((($this.0 as u64) $oper ($rhs.into_num()?.0 as u64)) as f64).into_object()
		}
	}
}

impl_type! { for Number, downcast_fn=downcast_num;
	fn "@num" (this) {
		this.into_object()
	}

	fn "@bool" (this) {
		(this.0 != 0.0).into_object()
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
	fn "**" (this, rhs) { this.0.powf(rhs.into_num()?.0).into_object() }

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

	fn "bitand" (this, rhs) { binary_oper!(bitwise; this & rhs) }
	fn "bitor" (this, rhs) { binary_oper!(bitwise; this | rhs) }
	fn "bitxor" (this, rhs) { binary_oper!(bitwise; this ^ rhs) }
	fn "bitshl" (this, rhs) { binary_oper!(bitwise; this << rhs) }
	fn "bitshr" (this, rhs) { binary_oper!(bitwise; this >> rhs) }
}








