use crate::err::{Result, Error};
use crate::object::{Object, AnyObject, Type, types::{RustFn}};
use std::collections::HashMap;
use crate::{map::Map, shared::Shared};


lazy_static::lazy_static! {
	pub static ref RUSTFN_MAP: Shared<dyn Map> = Shared::new({
		let mut map = HashMap::new();
		// map.insert(Object::new_variable("+"), Object::new_rustfn(number_add))
		map
	});
}

impl Type for RustFn {
	fn get_type_map() -> Shared<dyn Map> {
		RUSTFN_MAP.clone()
	}
}

