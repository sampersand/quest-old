use shared::Shared;
use obj::{Type, Object, types::{BoundFn, IntoObject}};

use std::fmt::Debug;
use std::hash::Hash;
use std::any::Any;

pub fn get_default_attr<T: 'static>(obj: &Object<T>, attr: &str) -> Option<BoundFn>
			where Object<T>: Type + ToString,
					T: Debug + PartialEq + Hash + Clone {
	let obj = obj.upgrade();

	match attr {
		"@text" => Some(obj.bind_to(|obj, _, _| Ok(obj.to_string().into_object()))),
		"clone" => Some(obj.bind_to(|obj, _, _| Ok(obj.duplicate()))),
		"@bool" => Some(BoundFn::bind_void(|_, _| Ok(true.into_object()))),
		"!" => Some(obj.bind_to(|obj, args, env| 
			Ok((!obj.attrs.into_bool(env)?).into_object())
			// let res = obj.attrs.call("@bool", args, env).expect("`@bool` is needed for `!`");
			// let read = res.read();
			// Ok(Object::<bool>::new(!read.downcast_ref::<bool>().expect("`@bool` didn't return a bool").data))
		)),
		"==" => Some(obj.bind_to(|obj, args, _| Ok(Object::<bool>::new(obj == &*args.get(0).expect("at least one arg is needed for `==`").read())))),
		"!=" => Some(obj.read().attrs.get("==").ok()?.bind_to(|obj, args, env| {
			let res = obj.attrs.call("()", args, env).expect("`()` is needed for `==` (for `!=`)");
			let read = res.read();
			Ok(read.attrs.call("!", args, env).expect("`!` needed for result of `==` (for `!=`)"))
		})),
		_ => None
	}
}