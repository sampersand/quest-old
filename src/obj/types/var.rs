use std::hash::{Hash, Hasher};
use shared::Shared;
use env::{Parent, Binding, Mapping};
use obj::{Object, AnyShared, SharedObject, types::Basic};

#[derive(Debug, Clone)]
enum Inner {
	Static(&'static str),
	Dynamic(String)
}

impl Object<Var> {
	pub fn new_var<I: Into<Var>>(var: I) -> Self {
		Object::new(var.into())
	}
}

impl AnyShared {
	pub fn cast_var(self) -> Result<SharedObject<Var>, Self> {
		unimplemented!();
	}
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Var(Inner);

impl From<String> for Var {
	fn from(var: String) -> Self {
		Var(Inner::Dynamic(var))
	}
}

impl From<&'static str> for Var {
	fn from(var: &'static str) -> Self {
		Var(Inner::Static(var))
	}
}

impl Eq for Inner {}
impl PartialEq for Inner {
	fn eq(&self, other: &Inner) -> bool {
		self.as_ref() == other.as_ref()
	}
}

impl Hash for Inner {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_ref().hash(h)
	}
}

impl AsRef<str> for Inner {
	fn as_ref(&self) -> &str {
		match self {
			Inner::Static(s) => s,
			Inner::Dynamic(s) => s.as_ref()
		}
	}
}


impl Parent for Var {
	fn binding() -> Shared<Binding> {
		DEFAULT_ATTRS.clone()
	}
}

lazy_static! {
	static ref DEFAULT_ATTRS: Shared<Binding> = Binding::new(Basic::binding(), {
		let mut h = Mapping::new();
		h
	});
}