use std::fmt::{self, Display, Formatter};
use crate::object::{Object, AnyObject};
use crate::err::{Result, Error};
use std::hash::{Hasher, Hash};
use std::ops::Deref;
use std::collections::HashMap;

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
	pub fn new_list(map: ObjMap) -> Object<Map> {
		Object::new(Map::new(map))
	}
}

impl AnyObject {
	pub fn tp_map(&self) -> Result<Object<Map>> {
		self.call_attr("@map", &[])?
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
	"@map" => |obj, _| unimplemented!(),
	"@list" => |obj, _| unimplemented!(),
	"@bool" => |obj, _| unimplemented!(),

	"==" => |obj, args| unimplemented!(),
	"len" => |obj, _| unimplemented!(),
	"+" => |obj, args| unimplemented!(),
	"-" => |obj, args| unimplemented!(),

	"[]" => |obj, args| unimplemented!(),
	"[]=" => |obj, args| unimplemented!(),

	"|" => |obj, args| unimplemented!(), // union
	"&" => |obj, args| unimplemented!(), // intersect
	"^" => |obj, args| unimplemented!(), // symmetric_difference
}


