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
		ObjectMap(RwLock::new(MapType::Uninit(T::get_type_map)))
	}

	pub fn initialized(map: Shared<dyn Map>) -> ObjectMap {
		ObjectMap(RwLock::new(MapType::Map(map)))
	}

	fn access_map<T, F: FnOnce(&Shared<dyn Map>) -> T>(&self, func: F) -> T {
		if let MapType::Map(ref map) = *self.0.read().expect("ObjMap.0 read failed") {
			return (func)(map)
		}

		let mut maptype = self.0.write().expect("ObjMap.0 write failed");

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