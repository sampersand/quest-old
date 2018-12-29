use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Var(&'static str);

impl Type for Var {
	fn create_mapping() -> Shared<dyn Mapping> {
		// lazy_static! {
		// 	static ref PARENT: Shared<Object> = Shared::new({
		// 		Object::new(crate::collections::Map::default())
		// 	});
		// }
		// Shared::new(ParentalMap::new_default(|| PARENT.clone()))

		lazy_static! {
			static ref PARENT: Shared<Object> = Shared::new(Object::new({
				fn at_text(args: &[&Shared<Object>]) -> crate::Result {
					Ok(TypedObject::new_text(
						args.get(0).ok_or_else(|| crate::Error::BadArgument("missing first arg", None))?
						   .downcast_var().ok_or_else(|| crate::Error::BadArgument("@text called with non-var argument", Some(args[0].clone())))?
						   .0.to_string()
					).objectify())
				}

				let mut map = crate::collections::Map::default();
				map.set(
					TypedObject::new_var("@text").objectify(),
					TypedObject::new_rustfn("Var.@text", at_text).objectify()
				);
				map
			}));
		}
		Shared::new(ParentalMap::new_default(|| PARENT.clone()))
	}
}

impl From<&'static str> for Var {
	fn from(id: &'static str) -> Var {
		Var(id)
	}
}

impl_typed_object!(Var, new_var, downcast_var);