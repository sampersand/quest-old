mod scope;
mod env;

pub use self::env::Environment;

use obj::{Object, AnyShared, types::IntoObject};
use std::collections::HashMap;

fn default_globals() -> HashMap<AnyShared, AnyShared> {
	let mut map = HashMap::new();
	map.insert("true".into_object() as AnyShared, true.into_object() as AnyShared);
	map.insert("false".into_object() as AnyShared, false.into_object() as AnyShared);
	map.insert("null".into_object() as AnyShared, Object::null());
	map
}