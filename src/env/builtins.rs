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
	// fn "__map__" (@this) { Object::new(this.map().clone()) }
	// fn "__env__" (@this) { Object::new(this.env().clone()) }

	// fn "::" (@this, key) {
	// 	this.get(key).ok_or_else(|| MissingKey {
	// 		key: key.clone(), obj: this.clone()
	// 	})?
	// }

	// fn "." (@this, key) {
	// 	// how is this different than `::` ?
	// 	unimplemented!()
	// }

	// fn ".=" (@this, key, val) {
	// 	this.map().write()
	// 	    .set(key.clone(), val.clone())
	// 	    .unwrap_or_else(Object::new_null)
	// }

	// fn ".~" (@this, key) {
	// 	this.map().write()
	// 	    .del(key)
	// 	    .unwrap_or_else(Object::new_null)
	// }

	// fn ".?" (@this, key) {
	// 	this.has(key).into_object()
	// }
}









