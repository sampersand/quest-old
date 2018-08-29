macro_rules! builtins {
	($(fn $name:ident($args:ident, $env:ident) $body:block)*) => {
		use obj_::classes_::boundfn::RustFn;
		$(
			pub const $name: RustFn = RustFn(concat!("<builtin ", stringify!($name), ">"), |_, $args, $env| $body);
		)*
	}
}

mod flow;
mod io;

use obj_::{Id, QObject__};
use std::collections::HashMap;


macro_rules! define_builtins {
	($($name:expr => $path:path),*) => {
		lazy_static! {
			static ref DEFAULT_BUILTINS: HashMap<Id, QObject__> = {
				let mut h: HashMap<Id, QObject__> = HashMap::new();
				$(
					h.insert($name.into(), $path.into_bound(&().into()).into());
				)*
				h
			};
		}
	}
}

define_builtins! {
	"if" => flow::IF,
	"while" => flow::WHILE,
	"return" => flow::RETURN,
	"exit" => flow::EXIT,
	"disp" => io::DISP,
	"prompt" => io::PROMPT
}


pub fn default_builtins() -> &'static HashMap<Id, QObject__> {
	&DEFAULT_BUILTINS
}