use std::any::Any;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use crate::object::{Type, Object, AnyObject};
use crate::shared::Shared;
use crate::map::{Map, ParentMap};
use crate::object::types::pristine::PRISTINE_MAP;


lazy_static! {
	pub static ref BASIC_MAP: Shared<dyn Map> = object_map!{UNTYPED "Basic", ParentMap::new(PRISTINE_MAP.clone(), HashMap::new());
		"===" => |obj, args| Ok(Object::new_boolean(obj.id() == getarg!(args[0])?.id())),
		"!==" => |obj, args| obj.call_attr("===", args)?.call_attr("!", &[]),
		"@bool" => |_, _| Ok(Object::new_boolean(true)),
		"@text" => |_, _| unimplemented!(),
		// "clone" => |obj, _| Ok(obj.duplicate())

		"==" => |obj, args| obj.call_attr("===", args),
		"!=" => |obj, args| obj.call_attr("==", args)?.call_attr("!", &[]),
		"->" => |obj, args| getarg!(args[0])?.call_attr("<-", &[obj]),

		"!" => |obj, _| obj.to_boolean()?.call_attr("!", &[]),
		"&&" => |obj, args| if obj.to_boolean()?.is_true() {
				getarg!(args[0]).map(Clone::clone)
			} else {
				Ok(obj.clone())
			},
		"||" => |obj, args| if obj.to_boolean()?.is_true() {
				Ok(obj.clone())
			} else {
				getarg!(args[0]).map(Clone::clone)
			},
	};
}