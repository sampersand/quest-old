use std::any::Any;
use lazy_static::lazy_static;
use crate::object::{Type, Object, AnyObject};
use crate::{shared::Shared, map::Map};
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};


lazy_static! {
	pub static ref BASIC_MAP: Shared<dyn Map> = Shared::new({
		let mut map = HashMap::<AnyObject, AnyObject>::new();
		map.insert(
			Object::new_variable("==="),
			Object::new(crate::object::types::RustFn::new_untyped(|val, args| {
				Ok(Object::new_boolean(val.id() == getarg!(args[0])?.id()))
			})));
		map
	});
}