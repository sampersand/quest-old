// use shared::Shared;
use obj::{AnyShared, AnyObject, AnyResult, Object, types::{HasDefaults, Type}};
use obj::types::Var;
use env::Environment;

// use std::fmt::Debug;
// use std::hash::Hash;
// use std::any::Any;

__impl_type! {
	defaults fn __get_default<T>(this, attr) where {
		::obj::Object<T>: HasDefaults + ToString, T: Type + Clone
	};

	fn _ () {
		__get_default_clone(this, attr.clone())
			.or_else(|| __get_default_tostring(this, attr.clone()))
			.or_else(|| __get_default_typed(this, attr))
	}
}

__impl_type! {
	defaults fn __get_default_tostring<T>(this, attr) where { ::obj::Object<T>: ToString, T: Send + Sync };

	fn "@text" (this) {
		Ok(this.read().to_string().into_object())
	}

	fn _ () {
		None
	}
}

__impl_type! {
	defaults fn __get_default_clone<T>(this, attr) where { T: Send + Sync + Clone };

	fn "clone" (this) env, {
		Ok(this.read().duplicate())
	}

	fn _ () {
		None
	}
}

__impl_type! {
	defaults fn __get_default_typed<T>(this, attr) where {
		::obj::Object<T>: HasDefaults, T: Type
	};

	static ref VAR_EQ: AnyShared = "==".into_anyshared();
	static ref VAR_NOT: AnyShared = "!=".into_anyshared();


	fn "and" (this, rhs) env, {
		if this.read_into_bool(env)? {
			Ok(rhs)
		} else {
			Ok(this)
		}
	}

	fn "or" (this, rhs) env, {
		if this.read_into_bool(env)? {
			Ok(this)
		} else {
			Ok(rhs)
		}
	}

	fn "@bool" (_) {
		Ok(true.into_object())
	}

	fn "not" (this) env, {
		Ok((!this.read_into_bool(env)?).into_object())
	}

	fn "==" (this, other) {
		let ref other = *other.read();
		Ok((&*this.read() == other).into_object())
	}

	fn "!=" (this) env, args, {
		this.read_call(&(VAR_EQ.clone()), args, env)?
			.read_call(&(VAR_NOT.clone()), &[], env)
	}

	fn "attrs" (this) {
		unimplemented!("Todo: attrs")
	}

	fn ".?" (this, attr) {
		Ok(this.read().attrs.has(&attr).into_object())
	}

	fn "." (this, attr) env, {
		this.read().attrs.get(&attr, env)
	}

	fn ".=" (this, attr, val) env, {
		this.write().attrs.set(attr, val.clone());
		Ok(val)
	}

	fn ".~" (this, attr) {
		this.write().attrs.del(&attr);
		Ok(this)
	}

	fn _ () {
		None
	}
}

pub fn get_default_tostring<T>(obj: &Object<T>, attr: AnyShared, env: &Environment) -> Option<AnyResult>
			where Object<T>: ToString, T: Send + Sync + 'static {
	match_defaults! {
		with obj attr ele _env;
		for<Var> { ele.data.id_str() } {
			"@text" => fn (this, {
				Ok(this.read().to_string().into_object())
			}),
			_ => None ()
		}
		_ => None ()
	}
}

pub fn get_default_eq<T>(obj: &Object<T>, attr: AnyShared, env: &Environment) -> Option<AnyResult>
			where T: Send + Sync + 'static + Eq {
	match_defaults! {
		with obj attr ele _env;
		for<Var> { ele.data.id_str() } {
			"==" => fn(this, (other) {
				let ref other = *other.read();
				Ok((&*this.read() == other).into_object())
			}),

			"!=" => fn(this, env, args, {
				this.read_call(&"==".into_anyshared(), args, env)?
					.read_call(&"!".into_anyshared(), &[], env)
			}),

			_ => None ()
		}
		_ => None ()
	}
}
pub fn get_default_clone<T>(obj: &Object<T>, attr: AnyShared, env: &Environment) -> Option<AnyResult>
			where T: Send + Sync + 'static + Clone {
	match_defaults! {
		with obj attr ele _env;
		for<Var> { ele.data.id_str() } {
			"clone" => fn (this, {
				Ok(this.read().duplicate())
			}),
			_ => None ()
		}
		_ => None ()
	}
}

pub fn get_default_norm<T: Send + Sync + 'static>(obj: &Object<T>, attr: AnyShared, env: &Environment) -> Option<AnyResult> {
	match_defaults!{
		with obj attr ele _env;
		for<Missing> { ele.data.id() } {
			attr => eval {
				Some(obj.attrs.get(&attr.into_anyshared(), env))
			}
		}

		for<Var> { ele.data.id_str() } {
			"and" => fn(this, env, args, (rhs) {
				if this.read_into_bool(env)? {
					Ok(rhs)
				} else {
					Ok(this)
				}
			}),

			"or" => fn(this, env, (rhs) {
				if this.read_into_bool(env)? {
					Ok(this)
				} else {
					Ok(rhs)
				}
			}),

			"@bool" => fn(_, {
				Ok(true.into_object())
			}),

			"!" => fn(this, env, {
				Ok((!this.read_into_bool(env)?).into_object())
			}),

			"attrs" => fn(this, {
				unimplemented!("Todo: attrs")
			}),

			".?" => fn(this, (attr) {
				Ok(this.read().attrs.has(&attr).into_object())
			}),

			"." => fn(this, env, (attr) {
				this.read().attrs.get(&attr, env)
			}),

			".=" => fn(this, env, (attr, val) {
				this.write().attrs.set(attr, val.clone());
				Ok(val)
			}),

			".~" => fn(this, (attr) {
				this.write().attrs.del(&attr);
				Ok(this)
			}),
			_ => None ()
		}
		_ => None ()
	}
}

pub fn get_default<T: Clone + Eq + Send + Sync + 'static>(obj: &Object<T>, attr: AnyShared, env: &Environment) -> Option<AnyResult>
			where Object<T>: ToString {
	get_default_norm(obj, attr.clone(), env)
		.or_else(|| get_default_tostring(obj, attr.clone(), env))
		.or_else(|| get_default_eq(obj, attr, env))
}








