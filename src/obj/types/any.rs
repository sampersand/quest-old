// use shared::Shared;
// use obj::{Type, Object, types::{BoundFn, IntoObject}};

// use std::fmt::Debug;
// use std::hash::Hash;
// use std::any::Any;

impl_type! {
	defaults fn get_default_attr<T>(this, attr) where {
		::obj::Object<T>: ::obj::types::Type + ToString,
		T: ::std::fmt::Debug + PartialEq + ::std::hash::Hash + Clone + Send + Sync
	};

	fn _ () {
		get_default_attr_clone(this, attr)
			.or_else(|| get_default_attr_tostring(this, attr))
			.or_else(|| get_default_attr_typed(this, attr))
	}
}

impl_type! {
	defaults fn get_default_attr_tostring<T>(this, attr) where { ::obj::Object<T>: ToString, T: Send + Sync };

	fn "@text" (this) {
		Ok(this.read().to_string().into_object())
	}

	fn _ () {
		None
	}
}

impl_type! {
	defaults fn get_default_attr_clone<T>(this, attr) where { T: Send + Sync + Clone };

	fn "clone" (this) {
		Ok(this.read().duplicate())
	}

	fn _ () {
		None
	}
}

impl_type! {
	defaults fn get_default_attr_typed<T>(this, attr) where {
		::obj::Object<T>: ::obj::types::Type,
		T: ::std::fmt::Debug + PartialEq + ::std::hash::Hash + Send + Sync + 'static
	};

	static ref VAR_EQ: SharedObject<Var> = "==".into_object();
	static ref VAR_NOT: SharedObject<Var> = "!=".into_object();

	fn "@bool" (_) {
		Ok(true.into_object())
	}

	fn "!" (this) env, {
		Ok((!this.read_into_bool(env)?).into_object() as AnyShared)
	}

	fn "==" (this, other) {
		let ref other = *other.read();
		Ok((&*this.read() == other).into_object() as AnyShared)
	}

	fn "!=" (this) env, args, {
		this.read_call(&(VAR_EQ.clone() as AnyShared), args, env)?
			.read_call(&(VAR_NOT.clone() as AnyShared), &[], env)
	}

	fn ".#" (this) {
		Ok(this.read().attrs.len().into_object() as AnyShared)
	}

	fn ".?" (this, attr) {
		Ok(this.read().attrs.has(&attr).into_object() as AnyShared)
	}

	fn "." (this, attr) {
		this.read().attrs.get(&attr)
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