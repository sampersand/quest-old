use std::fmt::{self, Display, Formatter};
use crate::object::{Object, AnyObject};
use crate::err::{Result, Error};
use std::hash::{Hasher, Hash};
use std::ops::Deref;
use std::collections::HashMap;

use crate::object::literal::consts::{self as literals,
	AT_MAP, AT_LIST, AT_TEXT, AT_BOOL,
	EQL,
	ADD, SUB, MUL,
	INDEX, INDEX_ASSIGN, INDEX_DELETE,
	B_OR, B_AND, B_XOR,
	L_LEN
};

type ObjMap = HashMap<AnyObject, AnyObject>;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Map(ObjMap);

impl Map {
	#[inline]
	pub fn new(list: ObjMap) -> Map {
		Map(list)
	}
}

impl Object<Map> {
	pub fn new_map(map: ObjMap) -> Object<Map> {
		Object::new(Map::new(map))
	}
}

impl AnyObject {
	pub fn tp_map(&self) -> Result<Object<Map>> {
		self.call_attr(literals::AT_MAP, &[])?
			.downcast_or_err::<Map>()
	}
}


impl AsRef<ObjMap> for Map {
	fn as_ref(&self) -> &ObjMap {
		&self.0
	}
}

impl Deref for Map {
	type Target = ObjMap;
	fn deref(&self) -> &ObjMap {
		&self.0
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

impl_type! { for Map; 
	AT_MAP => |obj, _| unimplemented!(),
	AT_LIST => |obj, _| unimplemented!(),
	AT_BOOL => |obj, _| unimplemented!(),
	AT_TEXT => |obj, _| unimplemented!(),

	EQL => |obj, args| unimplemented!(),
	L_LEN => |obj, _| unimplemented!(),
	ADD => |obj, args| unimplemented!(),
	SUB => |obj, args| unimplemented!(),

	INDEX => |obj, args| unimplemented!(),
	INDEX_ASSIGN => |obj, args| unimplemented!(),
	INDEX_DELETE => |obj, args| unimplemented!(),

	B_OR => |obj, args| unimplemented!(), // union
	B_AND => |obj, args| unimplemented!(), // intersect
	B_XOR => |obj, args| unimplemented!(), // symmetric_difference
}


