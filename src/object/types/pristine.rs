use std::any::Any;
use lazy_static::lazy_static;
use crate::object::{types::RustFn, Type, Object, AnyObject};
use crate::{shared::Shared, map::Map, err::Error};
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};


lazy_static! {
	pub static ref GETTER: Object<RustFn> = Object::new_named_untyped_rustfn("Pristine::`::`", |obj, args| {
		let attr = getarg!(args[0])?;
		obj.0.map.read().expect("read err in Pristine::`::`").get(attr).ok_or_else(|| Error::AttrMissing { obj: obj.clone(), attr: attr.clone()})
	});

	pub static ref PRISTINE_MAP: Shared<dyn Map> = object_map!{UNTYPED "Pristine", HashMap::new(); 
		"__id__" => |obj, _| Ok(Object::new_number(obj.id() as f64)),
		"__map__" => |obj, _| Ok(unimplemented!("map objects")),
		"__env__" => |obj, _| Ok(unimplemented!("map objects")),

		// there isn't actually a '::' thing here because it's not accessible ever

		"." => |obj, args| {
			/* i'm not sure how to get this to work */
			// we duplicate the object we get, then return that as a child
			// let val = obj.get(getarg!(args[0])?)?.duplicate();
			// Ok(Object::new_child(obj.))
			unimplemented!()
		},

		".=" => |obj, args| {
			obj.set(getarg!(args[0])?.clone(), getarg!(args[1])?.clone());
			Ok(getarg!(args[0])?.clone())
		},

		".~" => |obj, args| obj.del(getarg!(args[0])?),
		".?" => |obj, args| Ok(Object::new_boolean(obj.has(getarg!(args[0])?)))
		// "::" => |obj, args| GETTER.call_attr
		// GETTER.call_attr("()", obj.get(getarg!(args[0])?)
	};
}



