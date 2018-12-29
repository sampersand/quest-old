use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Num(i64);

macro_rules! impl_from {
	($($ty:ty)*) => {
		$(impl From<$ty> for Num {
			fn from(num: $ty) -> Num {
				Num(num as i64)
			}
		})*
	}
}

impl_from!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128);

impl_typed_object!(Num, new_num, downcast_num);

impl_type! { for Num;
	fn "+" (lhs, rhs) {
		
	}
}





// impl Type for Num {
// 	fn create_mapping() -> Shared<dyn Mapping> {
// 		lazy_static! {
// 			static ref PARENT: Shared<Object> = Shared::new(Object::new({
// 				fn at_text(args: &[&Shared<Object>]) -> crate::Result {
// 					Ok(TypedObject::new_text(
// 						args.get(0).ok_or_else(|| crate::Error::BadArgument("missing first arg", None))?
// 						   .downcast_num().ok_or_else(|| crate::Error::BadArgument("@text called with non-number argument", Some(args[0].clone())))?
// 						   .0.to_string()
// 					).objectify())
// 				}
// 				let mut map = crate::collections::Map::default();
// 				map.set(
// 					TypedObject::new_var("@text").objectify(),
// 					TypedObject::new_rustfn("Num.@text", at_text).objectify()
// 				);
// 				map
// 			}));
// 		}
// 		Shared::new(ParentalMap::new_default(PARENT.clone()))
// 	}
// }