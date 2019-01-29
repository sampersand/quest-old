use crate::Object;
use crate::collections::Mapping;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundObject {
	func: Option<Object>,
	parent: Object,
	key: Object
}

impl BoundObject {
	pub fn new(parent: Object, key: Object) -> BoundObject {
		BoundObject { func: parent.get(&key), parent, key }
	}
}


impl_typed_object!(BoundObject, _ , downcast_bound, is_bound);

impl_type! { for BoundObject, downcast_fn=downcast_bound;
	fn "()" (@this) args {
		if let Some(bound) = this.downcast_bound() {
			if let Some(func) = bound.func {
				let mut callargs = args.to_owned();
				assert!(callargs.len() >= 1, "callargs called without any args?");
				callargs[0] = &bound.parent;
				func.call_attr("()", &callargs)?
			} else {
				Object::new_null()
			}
		} else {
			unimplemented!("not a bound object")
		}
	}

	fn "func" (this) { this.func.clone().unwrap_or_else(Object::new_null) }
	fn "parent" (this) { this.parent.clone() }
	fn "key" (this) { this.key.clone() }

	fn "=" (this, val) {
		let mut this = this;
		this.parent.set(this.key, val.clone());
		val.clone()
	}


	fn "__missing__" (this, val) {
		use crate::collections::Mapping;
		println!("missing: {:?}, {:?}", this, val);
		this.func.and_then(|x| x.get(val)).ok_or(crate::Error::NothingToReturn)?
	}
	fn "@list" (this) { // this is a hack
		this.func.expect("@list err").call_attr("@list", &[])?
	}
	fn "@num" (this) { // this is a hack
		this.func.expect("@num err").call_attr("@num", &[])?
	}

}



