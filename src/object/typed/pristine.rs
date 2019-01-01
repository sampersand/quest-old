use crate::Object;
use lazy_static::lazy_static;

// to make it easier on my eyes
macro_rules! basic_map {
	($($args:tt)*) => {
		lazy_static! { 
			pub static ref PRISTINE_MAP: Object = function_map!(prefix="Pristine", downcast_fn=__error, $($args)*);
		}
	}
}

basic_map! {
	fn "__id__" (@this) { this.id().into_object() }
	fn "__map__" (@this) { Object::new(this.map().clone()) }
	fn "__env__" (@this) { Object::new(this.env().clone()) }

	fn "::" (@this, key) {
		this.get(key).ok_or_else(|| MissingKey {
			key: key.clone(), obj: this.clone()
		})?
	}

	fn "." (@_this, _key) {
		// how is this different than `::` ?
		todo!()
	}

	fn ".=" (@this, key, val) {
		this.map().write()
		    .set(key.clone(), val.clone())
		    .unwrap_or_else(Object::new_null)
	}

	fn ".~" (@this, key) {
		this.map().write()
		    .del(key)
		    .unwrap_or_else(Object::new_null)
	}

	fn ".?" (@this, key) {
		this.has(key).into_object()
	}
}