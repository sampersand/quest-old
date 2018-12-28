use crate::collections::{Mapping, ParentalMap};
pub use crate::collections::Map;

use crate::Shared;
use crate::object::{Object, IntoObject, Type};
use lazy_static::lazy_static;

// impl Type for Map {
// 	fn create_map() -> Shared<dyn Mapping> {
// 		lazy_static! {
// 			static ref CLASS: Shared<Object> = Shared::new({
// 				let mut m = Map::empty();
// 				// m.set("+", unimplemented!())
// 				m
// 			}.into());
// 		}

// 		Shared::new({
// 			let mut m = Map::empty();
// 			m.set("@parent".into_shared(), CLASS.clone());
// 			m
// 		}) as _
// 	}
// }

impl IntoObject for Map {
	fn into_object(self) -> Object {
		lazy_static! {
			static ref PARENT: Shared<dyn Mapping> = Shared::new({
				let mut m = Map::empty();
				// m.set("+", unimplemented!())
				m
			}) as _;
		}


		let parent = ParentalMap::new_mapped(PARENT.clone(), self);

		Object::new_mapped((), Shared::new(parent))
	}
}










