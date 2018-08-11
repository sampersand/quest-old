macro_rules! builtins {
	($(fn $name:ident($args:ident, $env:ident) $body:block)*) => {
		use obj::classes::boundfn::RustFn;
		$(
			pub const $name: RustFn = RustFn(concat!("<builtin ", stringify!($name), ">"), |_, $args, $env| $body);
		)*
	}
}

mod flow;
mod io;

use obj::{Id, QObject};
use std::collections::HashMap;


macro_rules! define_builtins {
	($($name:expr => $path:path),*) => {
		lazy_static! {
			static ref DEFAULT_BUILTINS: HashMap<Id, QObject> = {
				let mut h: HashMap<Id, QObject> = HashMap::new();
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


pub fn default_builtins() -> &'static HashMap<Id, QObject> {
	&DEFAULT_BUILTINS
}