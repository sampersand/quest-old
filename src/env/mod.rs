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
	map.insert("true".into_object() as AnyShared, true.into_object() as AnyShared);
	map.insert("false".into_object() as AnyShared, false.into_object() as AnyShared);
	map.insert("null".into_object() as AnyShared, Object::null());
	map.insert("if".into_object() as AnyShared, builtins::if_fn().into_object() as AnyShared);
	map.insert("disp".into_object() as AnyShared, builtins::disp_fn().into_object() as AnyShared);
	map.insert("rand".into_object() as AnyShared, builtins::rand_fn().into_object() as AnyShared);
	map.insert("prompt".into_object() as AnyShared, builtins::prompt_fn().into_object() as AnyShared);
	map.insert("while".into_object() as AnyShared, builtins::while_fn().into_object() as AnyShared);
	map
}