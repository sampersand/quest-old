use crate::{Shared, Mapping};
use crate::object::{Object, Type, IntoObject, r#type::Map};
use lazy_static::lazy_static;

pub use crate::collections::List;

impl Type for List {
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
