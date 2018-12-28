use crate::{Shared, Map, Environment, Object};

use std::fmt::Debug;

pub trait Type : Eq + Debug + Clone + 'static {
	fn create_map() -> Shared<Map>;
	fn into_object(self, env: Shared<Environment>) -> Object where Self: Sized {
		Object::new(self, env)
	}
}