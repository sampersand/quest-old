use crate::err::{Result, Error};
use crate::object::{Object, AnyObject, Type, types::{Number, RustFn}};
use std::collections::HashMap;
use crate::{map::Map, shared::Shared};

fn number_add(num: &Object<Number>, args: &[&AnyObject]) -> Result<AnyObject> {
	let arg = args.get(0).unwrap().downcast::<Number>().unwrap();
	Ok(Object::new_number(num.data().as_ref() + arg.data().as_ref()))
}

lazy_static::lazy_static! {
	pub static ref NUMBER_MAP: Shared<dyn Map> = Shared::new({
		let mut map = HashMap::<AnyObject, AnyObject>::new();
		map.insert(Object::new_variable("+"), Object::new_named_rustfn("+", number_add));
		map
	});
}

impl Type for  Number {
	fn get_type_map() -> Shared<dyn Map> {
		NUMBER_MAP.clone()
	}
}

