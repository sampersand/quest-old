use crate::{Shared, Mapping};
use crate::object::{Object, Type, IntoObject, r#type::Map};

use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Number {
	num: i64
}

impl Default for Number {
	fn default() -> Number {
		Number::from(0)
	}
}

macro_rules! impl_from_int {
	($($ty:ty)*) => {
		$(impl From<$ty> for Number {
			fn from(num: $ty) -> Number {
				Number { num: num as i64 }
			}
		}
		)*
	}
}

impl_from_int!{
	i8 i16 i32 i64 i128 isize
	u8 u16 u32 u64 u128 usize
}

impl IntoObject for Number {
	fn into_object(self) -> Object {
		self.into()
	}
}

impl Type for Number {
	fn create_map() -> Shared<dyn Mapping> {
		lazy_static! {
			static ref CLASS: Shared<Object> = {
				let mut m = Map::empty();
				// m.set("+", unimplemented!())
				m
			}.into_shared();
		}

		Shared::new({
			let mut m = Map::empty();
			m.set("@parent".into_shared(), CLASS.clone());
			m
		}) as _
	}
}