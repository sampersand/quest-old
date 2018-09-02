use env::Environment;
use shared::Shared;
use obj::{Id, AnyObject, AnyShared, AnyResult, SharedResult, Result, Error, WeakObject, Object, SharedObject};
use obj::types::{IntoObject, BoundFn, Number, Text, Var, List, Map};
use std::collections::HashMap;
use std::borrow::Borrow;
use std::fmt::{self, Debug, Formatter};

#[derive(Clone)]
pub struct Attributes {
	pub(super) obj: WeakObject,
	pub(super) map: HashMap<AnyShared, AnyShared>,
	pub(super) defaults: fn(&AnyObject, &AnyShared) -> Option<BoundFn>
}

lazy_static! {
	static ref VAR_CALL: SharedObject<Var> = Id::from("()").into_object();
	static ref VAR_NUM: SharedObject<Var> = Id::from("@num").into_object();
	static ref VAR_MAP: SharedObject<Var> = Id::from("@map").into_object();
	static ref VAR_LIST: SharedObject<Var> = Id::from("@list").into_object();
	static ref VAR_TEXT: SharedObject<Var> = Id::from("@text").into_object();
	static ref VAR_BOOL: SharedObject<Var> = Id::from("@bool").into_object();
}


impl Attributes {
	fn upgrade(&self) -> AnyShared {
		self.obj.upgrade().expect("attributes without an associated object?")
	}

	pub fn len(&self) -> usize {
		// todo: defaults length here
		self.map.len()
	}

	pub fn is_empty(&self) -> bool {
		self.map.is_empty()
	}

	pub fn get(&self, attr: &AnyShared) -> Result<AnyShared> {
		// todo: convert from `str` to `Var` to allow for `map` overriding this
		let obj = self.upgrade();
		let attr = attr.borrow();

		if let Some(attr) = self.map.get(attr) {
			Ok(attr.clone())
		} else {
			(self.defaults)(&*obj.read(), attr)
				.map(|bound| Object::new(bound) as AnyShared)
				.ok_or_else(|| Error::MissingAttr { obj: obj.clone(), attr: attr.clone() })

		}
	}

	pub fn call(&self, attr: &AnyShared, args: &[AnyShared], env: &Environment) -> AnyResult {
		// todo: convert from `str` to `Var` to allow for `map` overriding this
		let func = self.get(attr)?;
		let r = func.read();
		if let Some(bound) = r.downcast_ref::<BoundFn>() {
			bound.data.call_bound(args, env)
		} else {
			r.attrs.call(&(VAR_CALL.clone() as AnyShared), args, env)
		}
	}

	pub fn set(&mut self, attr: AnyShared, val: AnyShared) -> Option<AnyShared> {
		self.map.insert(attr, val)
	}

	pub fn has(&self, attr: &AnyShared) -> bool {
		unimplemented!()
		// self.defaults.contains_key(attr.borrow())
	}

	pub fn del(&mut self, attr: &AnyShared) -> Option<AnyShared> {
		self.map.remove(attr.borrow())
	}
}

impl Attributes {
	pub fn into_num(&self, env: &Environment) -> Result<Number> {
		// self.as_num(env).map(|x| x.read().data)
		unimplemented!()
	}

	pub fn as_num(&self, env: &Environment) -> SharedResult<Number> {
		// self.call("@num", &[], env).map(|x| x.read().try_upgrade::<Number>().expect("`@num` didn't return a number"))
		unimplemented!()
	}

	pub fn into_bool(&self, env: &Environment) -> Result<bool> {
		// self.as_bool(env).map(|x| x.read().data)
		unimplemented!()
	}

	pub fn as_bool(&self, env: &Environment) -> SharedResult<bool> {
		// self.call("@bool", &[], env).map(|x| x.read().try_upgrade::<bool>().expect("`@bool` didn't return a boolean"))
		unimplemented!()
	}

	pub fn into_string(&self, env: &Environment) -> Result<Text> {
		// self.as_string(env).map(|x| x.read().data.clone())
		unimplemented!()
	}

	pub fn as_string(&self, env: &Environment) -> SharedResult<Text> {
		// self.call("@text", &[], env).map(|x| x.read().try_upgrade::<Text>().expect("`@bool` didn't return a boolean"))
		unimplemented!()
	}
}


impl AnyShared {
	pub fn read_execute(&self, args: &[AnyShared], env: &Environment) -> AnyResult {
		if let Some(func) = self.read().downcast_ref::<BoundFn>() {
			return func.data.call_bound(args, env);
		}
		// this way we no longer are within the `read` lock as before
		self.read_call(&(VAR_CALL.clone() as AnyShared), args, env)
	}
}

impl<T: ?Sized> SharedObject<T> {
	pub fn read_get(&self, attr: &AnyShared, env: &Environment) -> AnyResult {
		self.read().attrs.call(&(".".into_object() as AnyShared), &[attr.clone()], env) // this might not
	}

	pub fn read_call(&self, attr: &AnyShared, args: &[AnyShared], env: &Environment) -> AnyResult {
		self.read_get(attr, env)?.read_execute(args, env)
		// func.
		// let f = func.read();
		// if let Some(bound) = f.downcast_ref::<BoundFn>() {
		// 	return bound.data.call_bound(args, env);
		// } 
		// drop(f);
		// func.read_call(&(VAR_CALL.clone() as AnyShared), args, env)
	}

	pub fn read_into_list(&self, env: &Environment) -> Result<List> {
		let func = self.read().attrs.get(&(VAR_LIST.clone() as AnyShared))?;
		let res = func.read_call(&(VAR_CALL.clone() as AnyShared), &[], env)?;
		let n = res.read().try_upgrade::<List>().expect("`@list` didn't return a number");
		let r2 = n.read();
		Ok(r2.data.clone())
	}

	pub fn read_into_map(&self, env: &Environment) -> Result<Map> {
		let func = self.read().attrs.get(&(VAR_MAP.clone() as AnyShared))?;
		let res = func.read_call(&(VAR_CALL.clone() as AnyShared), &[], env)?;
		let n = res.read().try_upgrade::<Map>().expect("`@map` didn't return a number");
		let r2 = n.read();
		Ok(r2.data.clone())
	}

	pub fn read_into_num(&self, env: &Environment) -> Result<Number> {
		let func = self.read().attrs.get(&(VAR_NUM.clone() as AnyShared))?;
		let res = func.read_call(&(VAR_CALL.clone() as AnyShared), &[], env)?;
		let n = res.read().try_upgrade::<Number>().expect("`@num` didn't return a number");
		let r2 = n.read();
		Ok(r2.data)
	}

	pub fn read_into_bool(&self, env: &Environment) -> Result<bool> {
		let func = self.read().attrs.get(&(VAR_BOOL.clone() as AnyShared))?;
		let res = func.read_call(&(VAR_CALL.clone() as AnyShared), &[], env)?;
		let n = res.read().try_upgrade::<bool>().expect("`@num` didn't return a number");
		let r2 = n.read();
		Ok(r2.data)
	}

	pub fn read_into_text(&self, env: &Environment) -> Result<Text> {
		let func = self.read().attrs.get(&(VAR_TEXT.clone() as AnyShared))?;
		let res = func.read_call(&(VAR_CALL.clone() as AnyShared), &[], env)?;
		let n = res.read().try_upgrade::<Text>().expect("`@num` didn't return a number");
		let r2 = n.read();
		Ok(r2.data.clone())
	}
}

impl Debug for Attributes {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.map, f)
	}
}




