mod ops;
mod data;
pub mod r#type;

use self::data::Data;
use self::ops::Ops;
use self::r#type::{Type, IntoObject};

use crate::{Shared, Environment, Mapping};
use std::fmt::{self, Debug, Formatter};

// Note:
// because the user could edit how things are hashed whenever they want, i've decided to forgo hashing for now. Plus, how do you hash a hashmap
pub struct Object {
	data: Data,
	ops: Ops,
	map: Shared<dyn Mapping>,
	bound_env: Shared<Environment>
}

impl Object {
	pub fn new<T: Type>(data: T) -> Object {
		trace!("Object being made for: {:?}", data);
		Object {
			data: Data::new(data),
			ops: Ops::from::<T>(),
			map: T::create_map(),
			bound_env: Environment::current()
		}
	}

	pub fn shared(self) -> Shared<Object> {
		Shared::new(self)
	}
}

impl Clone for Object {
	fn clone(&self) -> Object {
		trace!("Object being cloned");
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