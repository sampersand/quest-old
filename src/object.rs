mod ops;
mod data;

use self::data::Data;
use self::ops::Ops;

use crate::{Shared, Environment, Map};
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};

pub struct Object {
	data: Data,
	ops: Ops,
	map: Shared<Map>,
	bound_env: Shared<Environment>
}

impl Clone for Object {
	fn clone(&self) -> Object {
		Object { 
			data: (self.ops.clone)(&self.data),
			ops: self.ops,
			map: self.map.clone(), // should this be deep-copy
			bound_env: self.bound_env.clone()
		}
	}
}

impl Eq for Object {}
impl PartialEq for Object {
	fn eq(&self, other: &Object) -> bool {
		// todo: make this call `map.==` for all but variables, and self.ops.eq for variables
		(self.ops.eq)(&self.data, &other.data)
	}
}

impl Debug for Object {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		struct DataDebug<'a>(&'a Object);
		impl Debug for DataDebug<'_> {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				(self.0.ops.debug)(&self.0.data, f)
			}
		}

		f.debug_struct("Object")
		 .field("data", &DataDebug(self))
		 .field("ops", &self.ops)
		 .field("map", &self.map)
		 .field("bound_env", &self.bound_env)
		 .finish()
	}
}

impl Hash for Object {
	fn hash<H: Hasher>(&self, h: &mut H) {
		// todo: make this call `map.hash` for all but variables, adn self.ops.hash for variables
		// or do i? i dont want hash conflicts
		(self.ops.hash)(&self.data, h)
	}
}








