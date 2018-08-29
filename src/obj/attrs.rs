use env::Environment;
use shared::Shared;
use obj::{Id, AnyObject, AnyShared, AnyResult, SharedResult, Result, Error, WeakObject, Object, SharedObject};
use obj::types::{IntoObject, BoundFn, Number};
use std::collections::HashMap;
use std::borrow::Borrow;
use std::fmt::{self, Debug, Formatter};

pub struct Attributes {
	pub(super) obj: WeakObject,
	pub(super) map: HashMap<AnyShared, AnyShared>,
	pub(super) defaults: fn(&AnyObject, &str) -> Option<BoundFn>
}



impl Attributes {
	fn upgrade(&self) -> AnyShared {
		self.obj.upgrade().expect("attributes without an associated object?")
	}

	pub fn is_empty(&self) -> bool {
		self.map.is_empty()
	}

	pub fn get(&self, attr: &str) -> Result<AnyShared> {
		// todo: convert from `str` to `Var` to allow for `map` overriding this
		let obj = self.upgrade();
		let attr_obj = attr.into_anyobject();

		if let Some(attr) = self.map.get(&attr_obj) {
			Ok(attr.clone())
		} else {
			(self.defaults)(&*obj.read(), attr)
				.map(|bound| Object::new(bound) as AnyShared)
				.ok_or_else(|| Error::MissingAttr { obj: obj.clone(), attr: attr_obj })

		}
	}

	pub fn call(&self, attr: &'static str, args: &[&AnyShared], env: &mut Environment) -> AnyResult {
		// todo: convert from `str` to `Var` to allow for `map` overriding this
		let func = self.get(attr)?;
		let r = func.read();
		if let Some(bound) = r.downcast_ref::<BoundFn>() {
			bound.call_bound(args, env)
		} else {
			r.attrs.call("()", args, env)
		}
	}

	pub fn set(&mut self, attr: AnyShared, val: AnyShared) -> Option<AnyShared> {
		self.map.insert(attr, val)
	}

	pub fn has<Q: Borrow<Id>>(&self, attr: &Q) -> bool {
		unimplemented!()
		// self.defaults.contains_key(attr.borrow())
	}

	pub fn del(&mut self, attr: &AnyShared) -> Option<AnyShared> {
		self.map.remove(attr)
	}
}

impl Attributes {
	pub fn into_num(&self, env: &mut Environment) -> Result<Number> {
		self.as_num(env).map(|x| x.read().data)
	}

	pub fn as_num(&self, env: &mut Environment) -> SharedResult<Number> {
		self.call("@num", &[], env).map(|x| x.read().try_upgrade::<Number>().expect("`@num` didn't return a number"))
	}

	pub fn into_bool(&self, env: &mut Environment) -> Result<bool> {
		self.as_bool(env).map(|x| x.read().data)
	}

	pub fn as_bool(&self, env: &mut Environment) -> SharedResult<bool> {
		self.call("@bool", &[], env).map(|x| x.read().try_upgrade::<bool>().expect("`@bool` didn't return a boolean"))
	}

	pub fn into_string(&self, env: &mut Environment) -> Result<String> {
		self.as_string(env).map(|x| x.read().data.clone())
	}

	pub fn as_string(&self, env: &mut Environment) -> SharedResult<String> {
		self.call("@text", &[], env).map(|x| x.read().try_upgrade::<String>().expect("`@bool` didn't return a boolean"))
	}
}

impl Debug for Attributes {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.map, f)
	}
}




