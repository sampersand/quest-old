use crate::object::{ Type, types::Variable};
use std::collections::HashMap;
use crate::{map::Map, shared::Shared};


lazy_static::lazy_static! {
	pub static ref VARIABLE_MAP: Shared<dyn Map> = Shared::new({
		let map = HashMap::new();
		map
	});
}

impl Type for Variable {
	fn get_type_map() -> Shared<dyn Map> {
		VARIABLE_MAP.clone()
	}
}

