use std::any::Any;
use lazy_static::lazy_static;
use crate::object::{literals, Type, Object, AnyObject, types::RustFn};
use crate::{shared::Shared, map::Map, err::Error};
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

mod funcs {
	use crate::map::Map as MapTrait;
	use crate::err::{Error, Result};
	use crate::object::types::{Number, Boolean, Map};
	use crate::object::{literals, AnyObject, Object};

	pub fn __id__(obj: &AnyObject) -> Object<Number> {
		Object::new_number(obj.id() as _)
	}

	pub fn __map__(obj: &AnyObject) -> Object<Map> {
		// Object::new_map(obj.map())
		unimplemented!("__map__")
	}

	pub fn __env__(obj: &AnyObject) -> Object<Map> {
		unimplemented!("__env__")
	}

	pub fn colon_colon(obj: &AnyObject, attr: &AnyObject) -> Result<AnyObject>{
		obj.0.map.read()
			.expect(const_concat!("read err in Pristine::`", literals::COLON_COLON, "`"))
			.get(attr).ok_or_else(|| Error::AttrMissing { obj: obj.clone(), attr: attr.clone()})
	}

	pub fn access(obj: &AnyObject, attr: &AnyObject) -> Result<AnyObject> {
		Ok(obj.get(attr)?.duplicate_add_parent(obj.clone()))
	}

	pub fn access_assign(obj: &AnyObject, attr: AnyObject, val: AnyObject) -> AnyObject {
		obj.set(attr, val.clone());
		val
	}

	pub fn access_delete(obj: &AnyObject, attr: &AnyObject) -> Result<AnyObject> {
		obj.del(attr)
	}

	pub fn access_has(obj: &AnyObject, attr: &AnyObject) -> Object<Boolean> {
		Object::new_boolean(obj.has(attr))
	}
}

// so we can have the GETTER object
fn colon_colon(obj: &AnyObject, args: &[&AnyObject]) -> crate::err::Result<AnyObject> {
	funcs::colon_colon(obj, getarg!(args[0])?)
}

lazy_static! {
	pub static ref GETTER: Object<RustFn> = Object::new_named_untyped_rustfn(const_concat!("Pristine::`", literals::COLON_COLON, "`"), colon_colon);

	pub static ref PRISTINE_MAP: Shared<dyn Map> = object_map!{UNTYPED "Pristine", HashMap::new(); 
		literals::L_ID => |o, _| Ok(funcs::__id__(o)),
		literals::L_MAP => |o, _| Ok(funcs::__map__(o)),
		literals::L_ENV => |o, _| Ok(funcs::__env__(o)),
		literals::COLON_COLON => colon_colon,
		literals::ACCESS => |o, a| funcs::access(o, getarg!(a[0])?),
		literals::ACCESS_ASSIGN => |o, a| Ok(funcs::access_assign(o, getarg!(a[0])?.clone(), getarg!(a[1])?.clone())),
		literals::ACCESS_DELETE => |o, a| funcs::access_delete(o, getarg!(a[0])?),
		literals::ACCESS_HAS => |o, a| Ok(funcs::access_has(o, getarg!(a[0])?))
	};
}


#[cfg(test)]
mod fn_tests {
	use super::*;

	define_blank!(struct Blank;);

	#[test]
	fn foo() {
		let ref obj = Object::new_number(1.0).as_any();
		let ref eql = funcs::access(obj, &Object::new_variable_testing("==").as_any()).unwrap();

		println!("{:?}", eql.call_attr(literals::CALL, &[&Object::new_number(1.0).as_any(), &Object::new_number(1.0).as_any()]));
		// println!("{:?}", funcs::access_assign(obj, Object::new_variable("one"), Object::new_number(1.0)));
		// println!("{:?}", funcs::access(obj, &Object::new_variable("one").as_any()));
		// println!("{:?}", funcs::access(obj, &Object::new_variable("==").as_any()));
		// println!("\n\n");
		unimplemented!()
	}
}














