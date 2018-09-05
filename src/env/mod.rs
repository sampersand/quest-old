mod env;
mod builtins;

pub use self::env::Environment;

use obj::{Object, AnyShared, Result, types::IntoObject};
use std::collections::HashMap;
use parse::Token;
use std::boxed::FnBox;


pub type Peeker<'a> = ::std::iter::Peekable<&'a mut dyn Iterator<Item=Token>>;

pub trait Executable {
	fn execute(self: Box<Self>, env: &Environment, iter: &mut Peeker) -> Result<()>;
}

impl<F: FnBox(&Environment, &mut Peeker) -> Result<()>> Executable for F {
	fn execute(self: Box<Self>, env: &Environment, iter: &mut Peeker) -> Result<()> {
		self.call_box((env, iter))
	}
}



fn default_globals() -> HashMap<AnyShared, AnyShared> {
	let mut map = HashMap::new();
	map.insert("true".into_anyshared(), true.into_anyshared());
	map.insert("false".into_anyshared(), false.into_anyshared());
	map.insert("null".into_anyshared(), Object::null());
	map.insert("if".into_anyshared(), builtins::if_fn().into_anyshared());
	map.insert("disp".into_anyshared(), builtins::disp_fn().into_anyshared());
	map.insert("rand".into_anyshared(), builtins::rand_fn().into_anyshared());
	map.insert("prompt".into_anyshared(), builtins::prompt_fn().into_anyshared());
	map.insert("while".into_anyshared(), builtins::while_fn().into_anyshared());
	map.insert("return".into_anyshared(), builtins::return_fn().into_anyshared());
	map
}