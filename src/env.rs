use crate::shared::Shared;
use crate::map::Map;
use crate::object::{AnyObject, Object};
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
	static ref ENVIRONMENT_STACK: RwLock<Vec<AnyObject>> = RwLock::new(Vec::new());
}

pub fn current() -> Option<AnyObject> {
	ENVIRONMENT_STACK.read().expect("read err in current").iter().last().cloned()
}

// fn new_environment() -> AnyObject {
// 	Object::new_map(crate::object::types::Map::default())
// }

// pub fn current() -> AnyObject {
// 	if let Some(env) = current() {
// 		return env;
// 	}

// 	let mut stack = ENVIRONMENT_STACK.write().expect("write err in current");
// 	if stack.is_empty() {
// 		stack.push(new_environment());
// 	}

// 	stack.iter().last().cloned().unwrap()
// }

pub fn push_environment(env: AnyObject) {
	ENVIRONMENT_STACK.write().expect("write err in push_environment").push(env)
}

pub fn pop_environment() -> Option<AnyObject> {
	ENVIRONMENT_STACK.write().expect("write err in pop_environment").pop()
}


