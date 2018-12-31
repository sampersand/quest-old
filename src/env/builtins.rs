use lazy_static::lazy_static;
use crate::{Object, Result, Error};

// to make it easier on my eyes
macro_rules! builtins {
	($($args:tt)*) => {
		lazy_static! { 
			pub static ref BUILTINS_MAP: Object = Object::new(crate::collections::ParentalMap::new_mapped(
				|| crate::object::typed::PRISTINE_MAP.clone(),
				function_map!(prefix="Baisc", downcast_fn=__error, $($args)*)
			));
		}
	}
}

builtins! {
	fn "if" (@cond, if_true; if_false=Object::new_null()) {
		if *cond.into_bool()?.as_ref() {
			if_true.clone()
		} else {
			if_false
		}
	}
	fn "while" (@cond, body) { todo!(); }
	fn "loop" (@body) { loop {body.call_attr("()", &[])?; } }
	fn "switch" (@case) args { todo!(); }
	fn "return" (_) { todo!(); } // exit === return

	fn "import" (@file) { todo!(); }

	fn "disp" (_) args { todo!(); }
	fn "input" (_) /* what to do for args? */ { todo!() }
	
	fn "rand" (_) { todo!(); }
}


