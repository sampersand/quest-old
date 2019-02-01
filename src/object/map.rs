use crate::map::Map;
use crate::shared::Shared;
use crate::object::{Type, AnyObject};
use std::sync::RwLock;

#[derive(Debug)]
enum MapType {
	Uninit(fn() -> Shared<dyn Map>),
	Map(Shared<dyn Map>)
}

#[derive(Debug)]
pub struct ObjectMap(RwLock<MapType>);

impl ObjectMap {
	pub fn from_type<T: Type>() -> ObjectMap {
		ObjectMap::from_func(T::get_type_map)
	}

	pub fn from_func(func: fn() -> Shared<dyn Map>) -> ObjectMap {
		ObjectMap(RwLock::new(MapType::Uninit(func)))
	}

	#[cfg_attr(feature = "ignore-unused", allow(unused))]
	pub fn initialized(map: Shared<dyn Map>) -> ObjectMap {
		ObjectMap(RwLock::new(MapType::Map(map)))
	}

	fn access_map<T, F: FnOnce(&Shared<dyn Map>) -> T>(&self, func: F) -> T {
		if let MapType::Map(ref map) = *self.0.read().expect("ObjMap.0 read failed in `access_map`") {
			return (func)(map)
		}

		let mut maptype = self.0.write().expect("ObjMap.0 write failed in `access_map`");

		match *maptype {
			MapType::Map(ref map) => (func)(map),
			MapType::Uninit(uninit) => {
				let map = (uninit)();
				let result = (func)(&map);
				*maptype = MapType::Map(map);
				result
			}
		}
	}

	pub fn is_initialized(&self) -> bool {
		match &*self.0.read().expect("ObjectMap.0 read failed in `is_initialized`") {
			MapType::Uninit(_) => false,
			MapType::Map(_) => true
		}
	}
}

impl Map for ObjectMap {
	fn get(&self, key: &AnyObject) -> Option<AnyObject> {
		self.access_map(|map| map.read().expect("Shared read failed in `get`").get(key))
	}

	fn set(&mut self, key: AnyObject, val: AnyObject) {
		self.access_map(|map| map.write().expect("Shared write failed in `get`").set(key, val))
	}

	fn del(&mut self, key: &AnyObject) -> Option<AnyObject> {
		self.access_map(|map| map.write().expect("Shared write failed in `get`").del(key))
	}

	fn has(&self, key: &AnyObject) -> bool {
		self.access_map(|map| map.read().expect("Shared read failed in `get`").has(key))
	}	
}



#[cfg(test)]
mod tests {
use super::*;
	use std::collections::HashMap;
	use crate::object::types::Number;

	#[test]
	fn new() {
		assert!(!ObjectMap::from_func(|| unreachable!()).is_initialized());
		let hash_map: HashMap<AnyObject, AnyObject> = HashMap::new();
		assert!(ObjectMap::initialized(Shared::new(hash_map.clone()) as _).is_initialized());
		assert!(!ObjectMap::from_type::<Number>().is_initialized());
	}

	// #[test]
}

