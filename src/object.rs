mod data_mapping;
mod ops;
pub mod r#type;

use self::data_mapping::DataMapping;
use self::r#type::{Type, IntoObject};
use self::ops::Ops;

use crate::{Shared, Environment};
use crate::collections::{Collection, Mapping};
use std::fmt::{self, Debug, Formatter};


pub type Object = Shared<dyn Mapping>;

// // Note:
// // because the user could edit how things are hashed whenever they want, i've decided to forgo hashing for now. Plus, how do you hash a hashmap
// pub struct Object {
// 	ops: Ops,
// 	map: Shared<dyn Mapping>,
// 	bound_env: Shared<Environment>
// }

// impl Object {
// 	pub fn new<T: Type>(data: T) -> Object {
// 		Object::new_mapped(data, T::create_map())
// 	}


// 	pub fn new_mapped<T>(data: T, map: Shared<dyn Mapping>) -> Object 
// 				where T: Eq + Debug + Clone + Send + Sync + 'static {
// 		trace!("Object being made for: {:?}", data);
// 		Object {
// 			data: Data::new(data),
// 			ops: Ops::from::<T>(),
// 			map,
// 			bound_env: Environment::current()
// 		}

// 	}

// 	pub fn shared(self) -> Shared<Object> {
// 		Shared::new(self)
// 	}
// }



// impl Collection for Object {
// 	fn len(&self) -> usize {
// 		self.map.read().len()
// 	}

// 	fn is_empty(&self) -> bool {
// 		self.map.read().is_empty()
// 	}
// }

// impl Mapping for Object {
// 	fn get(&self, key: &Shared<Object>) -> Option<Shared<Object>> {
// 		self.map.read().get(key)
// 	}

// 	fn set(&mut self, key: Shared<Object>, val: Shared<Object>) -> Option<Shared<Object>> {
// 		self.map.write().set(key, val)
// 	}

// 	#[inline]
// 	fn del(&mut self, key: &Shared<Object>) -> Option<Shared<Object>> {
// 		self.map.write().del(key)
// 	}

// 	#[inline]
// 	fn has(&self, key: &Shared<Object>) -> bool {
// 		self.map.read().has(key)
// 	}
// }

// impl Clone for Object {
// 	fn clone(&self) -> Object {
// 		trace!("Object being cloned");
// 		Object { 
// 			ops: self.ops,
// 			map: self.map.clone(), // should this be deep-copy
// 			bound_env: self.bound_env.clone()
// 		}
// 	}
// }

// impl Eq for Object {}
// impl PartialEq for Object {
// 	fn eq(&self, other: &Object) -> bool {
// 		// todo: make this call `map.==` for all but variables, and self.ops.eq for variables
// 		(self.ops.eq)(&self, &other)
// 	}
// }

// impl Debug for Object {
// 	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
// 		struct DataDebug<'a>(&'a Object);
// 		impl Debug for DataDebug<'_> {
// 			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
// 				(self.0.ops.debug)(&self.0, f)
// 			}
// 		}

// 		f.debug_struct("Object")
// 		 .field("data", &DataDebug(self))
// 		 .field("map", &self.map)
// 		 .field("bound_env", &self.bound_env)
// 		 .finish()
// 	}
// }
