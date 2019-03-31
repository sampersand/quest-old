use std::fmt::{self, Display, Formatter};
use crate::object::{Object, AnyObject};
use crate::error::{Result, Error};
use std::hash::{Hasher, Hash};
use std::ops::Deref;
use std::collections::HashMap;
use crate::object::literals;

type ObjMap = HashMap<AnyObject, AnyObject>;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Map(ObjMap);

impl Map {
	#[inline]
	pub fn new(map: ObjMap) -> Map {
		Map(map)
	}

	#[inline]
	pub fn empty() -> Map {
		Map::default()
	}
}

impl Object<Map> {
	pub fn new_map<T: Into<Map>>(map: T) -> Object<Map> {
		Object::new(map.into())
	}
}

impl AnyObject {
	pub fn to_map(&self) -> Result<Object<Map>> {
		self.call_attr(literals::AT_MAP, &[])?
			.downcast_or_err::<Map>()
	}
}

impl PartialEq<Map> for Object<Map> {
	fn eq(&self, rhs: &Map) -> bool {
		*self.data().read().expect("read error in Object<Map>::eq") == *rhs
	}
}


impl From<ObjMap> for Map {
	fn from(map: ObjMap) -> Map {
		Map::new(map)
	}
}

impl From<Map> for ObjMap {
	fn from(map: Map) -> ObjMap {
		map.0
	}
}

impl Hash for Map {
	fn hash<H: Hasher>(&self, h: &mut H) {
		// TODO: Hash for Map
		// technically, i can set them all to the same value
		12.hash(h)
	}
}

mod funcs {
	use super::*;
	use super::Map;
	use crate::error::Result;
	use crate::object::{literals, Object, AnyObject};
	use crate::object::types::{Boolean, Text, List};

	pub fn at_map(map: &Object<Map>) -> Object<Map> {
		map.duplicate()
	}

	pub fn at_list(map: &Object<Map>) -> Object<List> {
		// List
		unimplemented!()
	}
}
impl_type! { for Map; 
	literals::AT_MAP => |obj, _| unimplemented!(),
	literals::AT_LIST => |obj, _| unimplemented!(),
	literals::AT_BOOL => |obj, _| unimplemented!(),
	literals::AT_TEXT => |obj, _| unimplemented!(),

	literals::EQL => |obj, args| unimplemented!(),
	literals::L_LEN => |obj, _| unimplemented!(),
	literals::ADD => |obj, args| unimplemented!(),
	literals::SUB => |obj, args| unimplemented!(),

	literals::INDEX => |obj, args| unimplemented!(),
	literals::INDEX_ASSIGN => |obj, args| unimplemented!(),
	literals::INDEX_DELETE => |obj, args| unimplemented!(),

	literals::B_OR => |obj, args| unimplemented!(), // union
	literals::B_AND => |obj, args| unimplemented!(), // intersect
	literals::B_XOR => |obj, args| unimplemented!(), // symmetric_difference
}


