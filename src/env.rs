use std::collections::HashMap;
use obj::{Id, AnyObject};
use shared::SharedMap;

#[derive(Clone)]
pub struct Environment<'a> {
	locals: SharedMap<Id, AnyObject>,
	globals: SharedMap<Id, AnyObject>,
	// specials: &'static HashMap<Id, fn(&Environment) -> AnyObject>,
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