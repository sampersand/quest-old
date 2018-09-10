use shared::Shared;
use obj::{Object, AnyShared, SharedObject};
use obj::types::{BoundFn, Var, Basic};
use env::{Parent, Mapping, Binding};


impl Object<bool> {
	pub fn new_bool<B: Into<bool>>(val: B) -> Self {
		Object::new(val.into())
	}
}
impl AnyShared {
	pub fn cast_bool(self) -> Result<SharedObject<bool>, Self> {
		unimplemented!();
	}
}

impl Parent for bool {
	fn binding() -> Shared<Binding> {
		println!("called binding on bool: {:#?}", &*DEFAULT_ATTRS);
		DEFAULT_ATTRS.clone()
	}
}

lazy_static! {
	static ref DEFAULT_ATTRS: Shared<Binding> = Binding::new(Basic::binding(), {
		let mut h = Mapping::new();
		h
	});
}