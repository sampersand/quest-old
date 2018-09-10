use obj::{Object, AnyShared, Result, Error, types::{IntoObject, BoundFnOld}};
use std::collections::HashMap;
use env::Environment;
use std::cmp::Ordering;
use parse::Token;
use std::io::{self, BufRead};
use rand;


pub fn disp_fn() -> BoundFnOld {
	BoundFnOld::bind_void(|args, env| {
		if args.len() != 0 {
			print!("{}", args[0].read_into_text(env)?.as_str());
			for arg in &args[1..] {
				print!(" {}", arg.read_into_text(env)?.as_str());
			}
		}
		println!();
		Ok(args.into_object())
	})
}

pub fn if_fn() -> BoundFnOld {
	BoundFnOld::bind_void(|args, env| {
		let cond = args.get(0).expect("`cond` needed for `if`");
		let if_true = args.get(1).expect("`if_true` needed for `if`");
		let if_false = args.get(2);

		if cond.read_into_bool(env)? == true {
			Ok(if_true.clone())
		} else if let Some(if_false) = if_false {
			Ok(if_false.clone())
		} else {
			Ok(Object::null())
		}
	})
}

pub fn while_fn() -> BoundFnOld {
	BoundFnOld::bind_void(|args, env| {
		let cond = args.get(0).expect("`cond` needed for `while`");
		let body = args.get(1).expect("`body` needed for `while`");

		let mut last = Object::null();
		while cond.read_call(&"()".into_anyshared(), &[], env)?.read_into_bool(env)? == true {
			last = body.read_call(&"()".into_anyshared(), &[], env)?;
		}
		Ok(last)
	})
}

pub fn return_fn() -> BoundFnOld {
	BoundFnOld::bind_void(|args, env| {
		let ret_env = args.get(0).cloned().unwrap_or_else(|| env.get_module_env().clone().into_anyshared());
		let obj = args.get(1).cloned().unwrap_or_else(Object::null);

		let ret_env = ret_env.read().try_upgrade::<Environment>().expect("Can only return to bindings");
		if &ret_env.read().data == env {
			Ok(obj)
		} else {
			Err(Error::Return { env: ret_env, obj })
		}
	})
}

pub fn rand_fn() -> BoundFnOld {
	BoundFnOld::bind_void(|args, env| {
		let rand = (rand::random::<u32>() & (::std::u32::MAX >> 1)) as i32;

		Ok(rand.into_anyshared())
	})
}


pub fn prompt_fn() -> BoundFnOld {
	BoundFnOld::bind_void(|args, env| {
		disp_fn().call_bound(args, env)?;
		let stdin = io::stdin();
		let res = stdin.lock().lines().next()
			.transpose()
			.expect("io error encountered when prompting!")
			.map(IntoObject::into_anyshared)
			.unwrap_or_else(Object::null);
		Ok(res)
	})
}