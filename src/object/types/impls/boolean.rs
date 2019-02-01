use crate::err::{Result, Error};
use crate::object::{Object, AnyObject, Type, types::{Boolean, RustFn}};
use std::collections::HashMap;
use crate::{map::Map, shared::Shared};


lazy_static::lazy_static! {
	pub static ref BOOLEAN_MAP: Shared<dyn Map> = Shared::new({
		let mut map = HashMap::<AnyObject, AnyObject>::new();
		map
	});
}

impl Type for  Boolean {
	fn get_type_map() -> Shared<dyn Map> {
		BOOLEAN_MAP.clone()
	}
}

