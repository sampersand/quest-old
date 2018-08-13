use std::collections::HashMap;
use obj::{Id, SharedObject};
use shared::SharedMap;

#[derive(Clone)]
pub struct Environment<'a> {
	locals: SharedMap<Id, SharedObject>,
	globals: SharedMap<Id, SharedObject>,
	// specials: &'static HashMap<Id, fn(&Environment) -> SharedObject>,
	binding: Option<&'a Environment<'a>>,
	// tokens: Vec<&'static Token>
}

impl<'a> Default for Environment<'a> {
	fn default() -> Self {
		Environment {
			locals: SharedMap::empty(),
			globals: SharedMap::empty(),
			binding: None
		}
	}
}