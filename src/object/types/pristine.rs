use std::any::Any;
use lazy_static::lazy_static;
use crate::object::{types::RustFn, Type, Object, AnyObject};
use crate::{shared::Shared, map::Map, err::Error};
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

use super::quest_funcs::{
	L___ID__, L___MAP__, L___ENV__,
	ACCESS, ACCESS_ASSIGN, ACCESS_DELETE, ACCESS_HAS,
	COLON_COLON
};

lazy_static! {
	pub static ref GETTER: Object<RustFn> = Object::new_named_untyped_rustfn(const_concat!("Pristine::`", COLON_COLON, "`"), |obj, args| {
		let attr = getarg!(args[0])?;
		obj.0.map.read().expect(const_concat!("read err in Pristine::`", COLON_COLON, "`")).get(attr).ok_or_else(|| Error::AttrMissing { obj: obj.clone(), attr: attr.clone()})
	});

	pub static ref PRISTINE_MAP: Shared<dyn Map> = object_map!{UNTYPED "Pristine", HashMap::new(); 
		L___ID__ => |obj, _| Ok(Object::new_number(obj.id() as f64)),
		L___MAP__ => |obj, _| Ok(unimplemented!("map objects")),
		L___ENV__ => |obj, _| Ok(unimplemented!("map objects")),

		// there isn't actually a '::' thing here because it's not accessible ever
		// (although it might be useful for things like `.?`)

		ACCESS => |obj, args| {
			Ok(obj.get(getarg!(args[0])?)?.duplicate_add_parent(obj.clone()))
		},

		ACCESS_ASSIGN => |obj, args| {
			obj.set(getarg!(args[0])?.clone(), getarg!(args[1])?.clone());
			Ok(getarg!(args[0])?.clone())
		},

		ACCESS_DELETE => |obj, args| obj.del(getarg!(args[0])?),
		ACCESS_HAS => |obj, args| Ok(Object::new_boolean(obj.has(getarg!(args[0])?)))
	};
}



